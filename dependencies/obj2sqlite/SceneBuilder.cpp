// Copyright (C) 2016 Chris Liebert

#include "SceneBuilder.h"
#include "sqlite3.h"

SceneBuilder::SceneBuilder()
{
	startPosition = 0;
	cfg = new ConfigLoader("obj2sqlite.cfg");
}

SceneBuilder::~SceneBuilder()
{
	delete cfg;
}

void SceneBuilder::addMaterial(Material* material)
{
	if (materials.find(material->name) != materials.end()) {
		std::cerr << "Material " << material->name << " has already been defined" << std::endl;
		exit(4);
	}
	else {
		materials[material->name] = *material;
	}
}

void SceneBuilder::addSceneNode(SceneNode* sceneNode)
{
	if(!sceneNode)
	{
		std::cerr << "Unable to add null sceneNode" << std::endl;
	}
	else
	{
		sceneNodes.push_back(*sceneNode);
	}
}

std::vector<unsigned char> readFile(const char* filename)
		{
	// open the file:
	std::ifstream file(filename, std::ios::binary);
	if(!file.is_open())
	{
		std::cerr << "Unable to open " << filename << std::endl;
	}

	// read the data:
	return std::vector<unsigned char>((std::istreambuf_iterator<char>(file)),
			std::istreambuf_iterator<char>());
		}

// Used to check file extension
bool hasEnding (std::string const &fullString, std::string const &ending)
{
	if (fullString.length() >= ending.length())
	{
		return (0 == fullString.compare (fullString.length() - ending.length(), ending.length(), ending));
	}
	else
	{
		return false;
	}
}

void SceneBuilder::addTexture(const char* textureFileName)
{
	std::set<std::string>::const_iterator it = textures.find(textureFileName);

	if(it == textures.end())
	{
		textures.insert(std::string(textureFileName));
	}
}

void calcNormal(float N[3], float v0[3], float v1[3], float v2[3]) {
  float v10[3];
  v10[0] = v1[0] - v0[0];
  v10[1] = v1[1] - v0[1];
  v10[2] = v1[2] - v0[2];

  float v20[3];
  v20[0] = v2[0] - v0[0];
  v20[1] = v2[1] - v0[1];
  v20[2] = v2[2] - v0[2];

  N[0] = v20[1] * v10[2] - v20[2] * v10[1];
  N[1] = v20[2] * v10[0] - v20[0] * v10[2];
  N[2] = v20[0] * v10[1] - v20[1] * v10[0];

  float len2 = N[0] * N[0] + N[1] * N[1] + N[2] * N[2];
  if (len2 > 0.0f) {
    float len = sqrtf(len2);

    N[0] /= len;
    N[1] /= len;
  }
}

// Cross-platform directory separator
#ifdef _MSC_VER
    #define DIRECTORY_PATH_SEPARATOR "\\"
#else
    #define DIRECTORY_PATH_SEPARATOR "/"
#endif

void replaceSubStr(string& source, string& it, string& with)
{
    int pos ;
    do
    {
        pos = source.find(it);
        if (pos!=-1)  source.replace(pos, it.length(), with);
    }
    while (pos!=-1);
}

void replaceSubStr(string& source, const char* it, const char* with) {
	std::string its(it);
	std::string withs(with);
	replaceSubStr(source, its, withs);
}

bool SceneBuilder::addWavefront(const char* fileName, glm::mat4 matrix) {
	tinyobj::attrib_t attrib;
	std::vector<tinyobj::shape_t> shapes;
	std::vector<tinyobj::material_t> materialList;
	std::stringstream modelDirectory, fileNamePath;
	modelDirectory << cfg->getVar("model.directory");
	modelDirectory << DIRECTORY_PATH_SEPARATOR;
	fileNamePath << modelDirectory.str();
	fileNamePath << fileName;
	std::string err;
	bool ret = tinyobj::LoadObj(&attrib, &shapes, &materialList, &err,
			fileNamePath.str().c_str(), modelDirectory.str().c_str(), true);
	if (!err.empty()) {
		std::cerr << err << std::endl;
		return false;
	}

	// load diffuse textures
	for (size_t mi = 0; mi < materialList.size(); mi++) {
		tinyobj::material_t* mp = &materialList[mi];

		if (mp->diffuse_texname.length() > 0) {
			// Check for paths that are not valid (unix support for paths with \ or \\ instead of /
#ifndef _MSC_VER
			replaceSubStr(mp->diffuse_texname, "\\\\", "/");
			replaceSubStr(mp->diffuse_texname, "\\", "/");
#endif
			addTexture(mp->diffuse_texname.c_str());
		}

		Material m;
		memcpy((void*)& m.ambient, (void*)& mp->ambient[0], sizeof(float) * 3);
		memcpy((void*)& m.diffuse, (void*)&  mp->diffuse[0], sizeof(float) * 3);
		memcpy((void*)& m.emission, (void*)& mp->emission[0], sizeof(float) * 3);
		memcpy((void*)& m.specular, (void*)& mp->specular[0], sizeof(float) * 3);
		memcpy((void*)& m.transmittance, (void*)&mp->transmittance[0], sizeof(float) * 3);
		memcpy((void*)& m.illum, (void*)& mp->illum, sizeof(int));
		memcpy((void*)& m.ior, (void*)& mp->ior, sizeof(float));
		memcpy((void*)& m.shininess, (void*)& mp->shininess, sizeof(float));
		memcpy((void*)& m.dissolve, (void*)& mp->dissolve, sizeof(float));
		strncpy(m.name, mp->name.c_str(), MAX_MATERIAL_NAME_LENGTH);
		strncpy(m.ambientTexName, mp->ambient_texname.c_str(), MAX_MATERIAL_NAME_LENGTH);
		strncpy(m.diffuseTexName, mp->diffuse_texname.c_str(), MAX_MATERIAL_NAME_LENGTH);
		strncpy(m.normalTexName, mp->specular_highlight_texname.c_str(), MAX_MATERIAL_NAME_LENGTH);
		strncpy(m.specularTexName, mp->specular_texname.c_str(), MAX_MATERIAL_NAME_LENGTH);
		addMaterial(&m);
	}

	// Load data
	for (size_t s = 0; s < shapes.size(); s++) {
		SceneNode o;
		o.center[0] = 0.f;
		o.center[1] = 0.f;
		o.center[2] = 0.f;

		for (size_t f = 0; f < shapes[s].mesh.indices.size() / 3; f++) {
			tinyobj::index_t idx0 = shapes[s].mesh.indices[3 * f + 0];
			tinyobj::index_t idx1 = shapes[s].mesh.indices[3 * f + 1];
			tinyobj::index_t idx2 = shapes[s].mesh.indices[3 * f + 2];

			int currentMaterialId = shapes[s].mesh.material_ids[f];

			if ((currentMaterialId < 0)
					|| (currentMaterialId
							>= static_cast<int>(materialList.size()))) {
				// Invaid material ID. Use default material.
				currentMaterialId = materialList.size(); // Default material is added to the last item in `materialList`.
				std::cerr << "Invalid material index: " << currentMaterialId << " reverting to default material." << std::endl;
			}

			float diffuse[3];
			for (size_t i = 0; i < 3; i++) {
				diffuse[i] = materialList[currentMaterialId].diffuse[i];
			}
			float tc[3][2];
			if (attrib.texcoords.size() > 0) {
				assert(attrib.texcoords.size() > 2 * idx0.texcoord_index + 1);
				assert(attrib.texcoords.size() > 2 * idx1.texcoord_index + 1);
				assert(attrib.texcoords.size() > 2 * idx2.texcoord_index + 1);
				tc[0][0] = attrib.texcoords[2 * idx0.texcoord_index];
				tc[0][1] = 1.0f - attrib.texcoords[2 * idx0.texcoord_index + 1];
				tc[1][0] = attrib.texcoords[2 * idx1.texcoord_index];
				tc[1][1] = 1.0f - attrib.texcoords[2 * idx1.texcoord_index + 1];
				tc[2][0] = attrib.texcoords[2 * idx2.texcoord_index];
				tc[2][1] = 1.0f - attrib.texcoords[2 * idx2.texcoord_index + 1];
			} else {
				std::cerr << "Texture coordinates are not defined" << std::endl;
				return false;
			}

			float v[3][3];
			for (int k = 0; k < 3; k++) {
				int f0 = idx0.vertex_index;
				int f1 = idx1.vertex_index;
				int f2 = idx2.vertex_index;
				assert(f0 >= 0);
				assert(f1 >= 0);
				assert(f2 >= 0);

				v[0][k] = attrib.vertices[3 * f0 + k];
				v[1][k] = attrib.vertices[3 * f1 + k];
				v[2][k] = attrib.vertices[3 * f2 + k];

				// local object center mean calculation (stage 1)
				o.center[0] += v[0][k];
				o.center[1] += v[1][k];
				o.center[2] += v[2][k];
			}

			float n[3][3];
			if (attrib.normals.size() > 0) {
				int f0 = idx0.normal_index;
				int f1 = idx1.normal_index;
				int f2 = idx2.normal_index;
				assert(f0 >= 0);
				assert(f1 >= 0);
				assert(f2 >= 0);
				for (int k = 0; k < 3; k++) {
					n[0][k] = attrib.normals[3 * f0 + k];
					n[1][k] = attrib.normals[3 * f1 + k];
					n[2][k] = attrib.normals[3 * f2 + k];
				}
			} else {
				// compute geometric normal
				calcNormal(n[0], v[0], v[1], v[2]);
				n[1][0] = n[0][0];
				n[1][1] = n[0][1];
				n[1][2] = n[0][2];
				n[2][0] = n[0][0];
				n[2][1] = n[0][1];
				n[2][2] = n[0][2];
			}

			for (int k = 0; k < 3; k++) {
				Vertex vert;
				vert.position[0] = v[k][0];
				vert.position[1] = v[k][1];
				vert.position[2] = v[k][2];
				vert.normal[0] = n[k][0];
				vert.normal[1] = n[k][1];
				vert.normal[2] = n[k][2];
				vert.textureCoordinate[0] = tc[k][0];
				vert.textureCoordinate[1] = tc[k][1];
				o.vertexData.push_back(vert);
				//indices.push_back((unsigned) indices.size());
			}
		}

		if(o.vertexData.size() == 0) {
			// Ignore scene nodes that don't have geometry
			std::cerr << "Warning, scene node " << shapes[s].name << " does not containing geometry, ommiting." << std::endl;
			continue;
		}

		o.name = shapes[s].name;

		// 2nd stage of mean calculation
		o.center[0] /= (float) shapes.size();
		o.center[1] /= (float) shapes.size();
		o.center[2] /= (float) shapes.size();

		o.radius = 0.f;
		// Radius calculation

		for(size_t i=0; i<o.vertexData.size(); i++) {
			float x = o.vertexData.at(i).position[0];
			float y = o.vertexData.at(i).position[1];
			float z = o.vertexData.at(i).position[2];
			float nx = x - o.center[0];
			float ny = y - o.center[1];
			float nz = z - o.center[2];
			float r2 = sqrtf(nx*nx + ny*ny + nz*nz);
			if(r2 > o.radius) {
				o.radius = r2;
			}
		}

		if (o.radius <= 0.f) {
			//std::cerr << "Warning, calculated bounding radius " << o.radius
			//		<< " for " << o.name << " using 0.1f instead" << std::endl;
			o.radius = 0.1f;
		}


		// OpenGL viewer does not support texturing with per-face material.
		if (shapes[s].mesh.material_ids.size() > 0
				&& shapes[s].mesh.material_ids.size() > s) {
			// Base case
			o.materialId = shapes[s].mesh.material_ids[s] + 1;
		} else {
			o.materialId = materialList.size(); // = ID for default material.
		}

		o.startPosition = startPosition;
		startPosition += o.vertexData.size();
		o.endPosition = o.startPosition + o.vertexData.size();
		sceneNodes.push_back(o);
	}


	if(sceneNodes.size() == 0)
	{
		std::cerr << "Error: No scene nodes defined in " << fileName << std::endl;
		return false;
	}

	return true;
}

string intToStr(int i)
{
	char buffer[33];
	snprintf(buffer, 33, "%i", i);
	string s(buffer);
	return s;
}

string fToStr(float& f)
{
	char buffer[33];
	snprintf(buffer, 33, "%f", f);
	return string(buffer);
}

int SceneBuilder::getMaterialId(string& s)
{
	int i = 1;
	for (std::map<string,Material>::iterator it=materials.begin(); it!=materials.end(); ++it)
	{
		if(s.compare(it->first) == 0)
		{
			return i;
		}
		i++;
	}
	return -1;
}

string loadTextFile(string filepath) {
	string contents = "";
	std::ifstream fileStream(filepath.c_str());
	if (fileStream.is_open())
	{
		std::string line;
		while (std::getline(fileStream, line))
		{
			contents += line;
			contents += "\n";
		}
		fileStream.close();
	}
	else
	{
		std::cerr << "Unable to load text file: " << filepath << std::endl;
	}
	return contents;
}

void SceneBuilder::saveToDB(const char* dbFile)
{
	// sqlite
	sqlite3* db = 0;
	sqlite3_stmt* stmt = 0;
	int rc;
	rc = sqlite3_open(dbFile, &db);
	char* error_msg = 0;
	if(rc)
	{
		//cerr << "Can't open database: " << sqlite3_errmsg(db) << endl;
		sqlite3_close(db);
		return ;
	}
	const char* dbScemeSQL =
			"BEGIN TRANSACTION;"\
			"DROP TABLE IF EXISTS vertex;"\
			"DROP TABLE IF EXISTS scene_node;"\
			"DROP TABLE IF EXISTS material;"\
			"DROP TABLE IF EXISTS texture;"\
			"CREATE TABLE vertex(id INTEGER PRIMARY KEY AUTOINCREMENT, px REAL NOT NULL, py REAL NOT NULL, pz REAL NOT NULL, nx REAL NOT NULL, ny REAL NOT NULL, nz REAL NOT NULL, tu REAL NOT NULL, tv REAL NOT NULL);"\
			"CREATE TABLE scene_node(id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, material_id INTEGER, start_position INTEGER NOT NULL, end_position INTEGER NOT NULL, radius REAL NOT NULL, center_x REAL NOT NULL, center_y REAL NOT NULL, center_z REAL NOT NULL);"\
			"CREATE TABLE material(id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, normal_texname TEXT, dissolve REAL, diffuse_r REAL, diffuse_g REAL, diffuse_b REAL, transmittance_r REAL, transmittance_g REAL, transmittance_b REAL, emission_r REAL, emission_g REAL, emission_b REAL, shininess REAL, specular_texname TEXT, specular_r REAL, specular_g REAL, specular_b REAL, diffuse_texname TEXT, ambient_r REAL, ambient_g REAL, ambient_b REAL, ior INTEGER, ambient_texname TEXT, illum INTEGER);" \
			"CREATE TABLE texture(name TEXT PRIMARY KEY NOT NULL, image BLOB NOT NULL);";

	rc = sqlite3_open(dbFile, &db);
	if(rc)
	{
		cerr << "Can't open database: " << sqlite3_errmsg(db) << endl;
		sqlite3_close(db);
		return;
	}

	rc = sqlite3_exec(db, dbScemeSQL, 0, 0, &error_msg);
	//rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
	if (rc != SQLITE_OK)
	{
		cerr << "SQL Scheme Creation Error: " << sqlite3_errmsg(db) << endl;
	}

	for(size_t i=0; i<sceneNodes.size(); i++)
	{
		for(size_t j=0; j<sceneNodes.at(i).vertexData.size(); j++)
		{
			// Add each vertex in the scene node
			string vertexInsertSQL = "INSERT INTO vertex(px, py, pz, nx, ny, nz, tu, tv) VALUES (";
			vertexInsertSQL += fToStr(sceneNodes.at(i).vertexData.at(j).position[0]);
			vertexInsertSQL += ",";
			vertexInsertSQL += fToStr(sceneNodes.at(i).vertexData.at(j).position[1]);
			vertexInsertSQL += ",";
			vertexInsertSQL += fToStr(sceneNodes.at(i).vertexData.at(j).position[2]);
			vertexInsertSQL += ",";
			vertexInsertSQL += fToStr(sceneNodes.at(i).vertexData.at(j).normal[0]);
			vertexInsertSQL += ",";
			vertexInsertSQL += fToStr(sceneNodes.at(i).vertexData.at(j).normal[1]);
			vertexInsertSQL += ",";
			vertexInsertSQL += fToStr(sceneNodes.at(i).vertexData.at(j).normal[2]);
			vertexInsertSQL += ",";
			vertexInsertSQL += fToStr(sceneNodes.at(i).vertexData.at(j).textureCoordinate[0]);
			vertexInsertSQL += ",";
			vertexInsertSQL += fToStr(sceneNodes.at(i).vertexData.at(j).textureCoordinate[1]);
			vertexInsertSQL += ");";
			rc = sqlite3_exec(db, vertexInsertSQL.c_str(), 0, 0, &error_msg);
			if (rc != SQLITE_OK)
			{
				cerr << "SQL Vertex Data Error: " << sqlite3_errmsg(db) << endl;
			}
		}

		string sceneNodeInsertSQL = "INSERT INTO scene_node(name, material_id, start_position, end_position, radius, center_x, center_y, center_z) VALUES (";
		sceneNodeInsertSQL += "'" + sceneNodes.at(i).name + "'";
		sceneNodeInsertSQL += ",";
		sceneNodeInsertSQL += intToStr(sceneNodes.at(i).materialId);
		sceneNodeInsertSQL += ",";
		sceneNodeInsertSQL += intToStr(sceneNodes.at(i).startPosition);
		sceneNodeInsertSQL += ",";
		sceneNodeInsertSQL += intToStr(sceneNodes.at(i).endPosition);
		sceneNodeInsertSQL += ",";
		sceneNodeInsertSQL += fToStr(sceneNodes.at(i).radius);
		sceneNodeInsertSQL += ",";
		sceneNodeInsertSQL += fToStr(sceneNodes.at(i).center[0]);
		sceneNodeInsertSQL += ",";
		sceneNodeInsertSQL += fToStr(sceneNodes.at(i).center[1]);
		sceneNodeInsertSQL += ",";
		sceneNodeInsertSQL += fToStr(sceneNodes.at(i).center[2]);
		sceneNodeInsertSQL += ");";
		rc = sqlite3_exec(db, sceneNodeInsertSQL.c_str(), 0, 0, &error_msg);
		if (rc != SQLITE_OK)
		{
			cerr << "SQL Scene Node Error: " << sqlite3_errmsg(db) << endl;
		}
	}

	int i = 0;
	for (std::map<string,Material>::iterator it=materials.begin(); it!=materials.end(); ++it)
	{
		string materialInsertSQL = "INSERT INTO material(name, normal_texname, dissolve, diffuse_r, diffuse_g, diffuse_b, "\
				"transmittance_r, transmittance_g, transmittance_b, "\
				"emission_r, emission_g, emission_b, shininess, specular_texname, "\
				"specular_r, specular_g, specular_b, diffuse_texname,"\
				"ambient_r, ambient_g, ambient_b, ior, ambient_texname, illum) "\
				"VALUES (";
		materialInsertSQL += "'" + it->first + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + string(it->second.normalTexName) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + fToStr(it->second.dissolve) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + fToStr(it->second.diffuse[0]) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + fToStr(it->second.diffuse[1]) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + fToStr(it->second.diffuse[2]) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + fToStr(it->second.transmittance[0]) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + fToStr(it->second.transmittance[1]) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + fToStr(it->second.transmittance[2]) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + fToStr(it->second.emission[0]) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + fToStr(it->second.emission[1]) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + fToStr(it->second.emission[2]) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + fToStr(it->second.shininess) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + string(it->second.specularTexName) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + fToStr(it->second.specular[0]) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + fToStr(it->second.specular[1]) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + fToStr(it->second.specular[2]) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + string(it->second.diffuseTexName) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + fToStr(it->second.ambient[0]) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + fToStr(it->second.ambient[1]) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + fToStr(it->second.ambient[2]) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + fToStr(it->second.ior) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + string(it->second.ambientTexName) + "'";
		materialInsertSQL += ",";
		materialInsertSQL += "'" + fToStr(it->second.ior) + "');";
		i++;
		rc = sqlite3_exec(db, materialInsertSQL.c_str(), 0, 0, &error_msg);
		if (rc != SQLITE_OK)
		{
			cerr << "SQL Material Error: " << sqlite3_errmsg(db) << endl;
		}
	}

	// Insert textures
	for (std::set<string>::iterator it=textures.begin(); it!=textures.end(); ++it)
	{
		string textureInsertSQL = "INSERT INTO texture(name, image) VALUES (?,?);";

		rc = sqlite3_prepare(db, textureInsertSQL.c_str(), -1, &stmt, 0);
		if( rc!=SQLITE_OK )
		{
			cerr << "Unable to prepare SQL: " << textureInsertSQL << endl;
		}

		// TODO: detect model and texture directories

		string fileName(cfg->getVar("texture.directory").c_str());
		fileName += DIRECTORY_SEPARATOR;
		fileName += *it;
		std::vector<unsigned char> imgContents = readFile(fileName.c_str());
		const unsigned char *zBlob = imgContents.data();
		int nBlob = (int) imgContents.size();
		std::string itstr = *it;
		sqlite3_bind_text(stmt, 1, itstr.c_str(), -1, SQLITE_STATIC);
		sqlite3_bind_blob(stmt, 2, zBlob, nBlob, SQLITE_STATIC);

		rc = sqlite3_step(stmt);
		assert( rc!=SQLITE_ROW );
		rc = sqlite3_finalize(stmt);
	}

	string textureInsertSQL = "INSERT INTO texture(name, image) VALUES (?,?);";

	rc = sqlite3_prepare(db, textureInsertSQL.c_str(), -1, &stmt, 0);
	if( rc!=SQLITE_OK )
	{
		cerr << "Unable to prepare SQL: " << textureInsertSQL << endl;
	}

	// Add DEFALUT_BLANK_TEXTURE.png
	string fileName(cfg->getVar("texture.directory").c_str());
	fileName += DIRECTORY_SEPARATOR;
	fileName += "DEFAULT_BLANK_TEXTURE.png";
	std::vector<unsigned char> imgContents = readFile(fileName.c_str());
	const unsigned char *zBlob = imgContents.data();
	int nBlob = (int) imgContents.size();
	sqlite3_bind_text(stmt, 1, "DEFAULT_BLANK_TEXTURE.png", -1, SQLITE_STATIC);
	sqlite3_bind_blob(stmt, 2, zBlob, nBlob, SQLITE_STATIC);
	rc = sqlite3_step(stmt);
	assert( rc!=SQLITE_ROW );
	rc = sqlite3_finalize(stmt);

	rc = sqlite3_exec(db, "COMMIT;", 0, 0, &error_msg);
	if (rc != SQLITE_OK)
	{
		cerr << "SQL Texture Error: " << sqlite3_errmsg(db) << endl;
	}

	cout << "Done writing database" << endl;
}

void wavefrontToSQLite(const char* wavefront, const char* database) {
	SceneBuilder sceneBuilder;
	bool success = sceneBuilder.addWavefront(wavefront, glm::translate(glm::mat4(1.f), glm::vec3(0.0, 0.0, 0.0)));
	if(success) {
		sceneBuilder.saveToDB(database);
	} else {
		std::cerr << "Unable to add wavefront " << wavefront << std::endl;
	}
}
