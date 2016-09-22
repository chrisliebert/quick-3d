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
		cerr << "Unable to open " << filename << endl;
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

void SceneBuilder::addTexture(const char* textureFileName, unsigned& textureId)
{
	std::map<std::string,int>::const_iterator it = textures.find(textureFileName);

	if(it == textures.end())
	{
		textures[textureFileName] = 0;
	}
}

void SceneBuilder::addWavefront(const char* fileName, glm::mat4 matrix)
{
	tinyobj::attrib_t attrib;
	std::vector<tinyobj::shape_t> shapes;
	std::vector<tinyobj::material_t> materials;
	std::string modelDirectory(cfg->getVar("model.directory"));
	modelDirectory += DIRECTORY_SEPARATOR;
	std::string fileNameStr(modelDirectory);
	fileNameStr += fileName;
	std::string err;
	bool ret = tinyobj::LoadObj(&attrib, &shapes, &materials, &err, fileNameStr.c_str(), modelDirectory.c_str(), true);
	if (!err.empty())
	{
		std::cerr << err << std::endl;
		return;
	}


	for (size_t i = 0; i<materials.size(); i++)
	{
		Material m;
		memcpy((void*)& m.ambient, (void*)& materials[i].ambient[0], sizeof(float) * 3);
		memcpy((void*)& m.diffuse, (void*)& materials[i].diffuse[0], sizeof(float) * 3);
		memcpy((void*)& m.emission, (void*)& materials[i].emission[0], sizeof(float) * 3);
		memcpy((void*)& m.specular, (void*)& materials[i].specular[0], sizeof(float) * 3);
		memcpy((void*)& m.transmittance, (void*)&materials[i].transmittance[0], sizeof(float) * 3);
		memcpy((void*)& m.illum, (void*)& materials[i].illum, sizeof(int));
		memcpy((void*)& m.ior, (void*)& materials[i].ior, sizeof(float));
		memcpy((void*)& m.shininess, (void*)& materials[i].shininess, sizeof(float));
		memcpy((void*)& m.dissolve, (void*)& materials[i].dissolve, sizeof(float));
		strcpy(m.name, materials[i].name.c_str());
		strcpy(m.ambientTexName, materials[i].ambient_texname.c_str());
		strcpy(m.diffuseTexName, materials[i].diffuse_texname.c_str());
		strcpy(m.normalTexName, materials[i].specular_highlight_texname.c_str());
		strcpy(m.specularTexName, materials[i].specular_texname.c_str());
		addMaterial(&m);
	}


	for (size_t i = 0; i < shapes.size(); i++)
	{
		std::vector<Vertex> mVertexData;

		unsigned int materialId, lastMaterialId;
		materialId = lastMaterialId = 0;
		if (shapes[i].mesh.material_ids.size() > 0)
		{
			materialId = lastMaterialId = shapes[i].mesh.material_ids[0];
		}

		for (size_t j = 0; j <shapes[i].mesh.indices.size(); j++)
		{
			if ((j % 3) == 0)
			{

				lastMaterialId = materialId;
				materialId = shapes[i].mesh.material_ids[j / 3];

				if (materialId != lastMaterialId)
				{
					SceneNode sceneNode;
					sceneNode.name = shapes[i].name.c_str();
					sceneNode.material = materials[lastMaterialId].name.c_str();
					sceneNode.vertexDataSize = mVertexData.size();
					sceneNode.vertexData = new Vertex[sceneNode.vertexDataSize];
					memcpy((void*)sceneNode.vertexData, (void*)mVertexData.data(), sizeof(Vertex) * sceneNode.vertexDataSize);
					sceneNode.startPosition = startPosition;
					startPosition += (unsigned)sceneNode.vertexDataSize;
					sceneNode.endPosition = (unsigned)(sceneNode.startPosition + sceneNode.vertexDataSize);
					sceneNode.primativeMode = 6;//GL_TRIANGLES;
					sceneNode.diffuseTextureId = 0;
					sceneNode.modelViewMatrix = matrix;
					sceneNode.radius = 0.f;
					addSceneNode(&sceneNode);
					mVertexData.clear();
				}
			}

			Vertex v;

			memcpy((void*)& v.vertex, (void*)& attrib.vertices[shapes[i].mesh.indices[j].vertex_index * 3], sizeof(float) * 3);

			if ((shapes[i].mesh.indices[j].normal_index * 3) >= attrib.normals.size())
			{
				//std::cout << "Calculating normal " << std::endl;

				// Points of the triangle
				glm::vec3 pointA;
				glm::vec3 pointB;
				glm::vec3 pointC;

				int triangleIndex = i % 3;
				if (triangleIndex == 0)
				{
					pointA = glm::vec3(v.vertex[0], v.vertex[1], v.vertex[2]);
					pointB = glm::vec3(
							attrib.vertices[3 + shapes[i].mesh.indices[j].vertex_index * 3],
							attrib.vertices[4 + shapes[i].mesh.indices[j].vertex_index * 3],
							attrib.vertices[5 + shapes[i].mesh.indices[j].vertex_index * 3]
					);
					pointC = glm::vec3(
							attrib.vertices[6 + shapes[i].mesh.indices[j].vertex_index * 3],
							attrib.vertices[7 + shapes[i].mesh.indices[j].vertex_index * 3],
							attrib.vertices[8 + shapes[i].mesh.indices[j].vertex_index * 3]
					);
					// Calculate normal
					glm::vec3 nu = pointB - pointA;
					glm::vec3 nv = pointC - pointA;
					v.normal[0] = (nu.y * nv.z) - (nu.z * nv.y);
					v.normal[1] = (nu.z * nv.x) - (nu.x * nv.z);
					v.normal[2] = (nu.x * nv.y) - (nu.y * nv.x);
				}
				else if (triangleIndex == 1)
				{
					v.normal[0] = attrib.vertices[shapes[i].mesh.indices[j].normal_index * 3 - 3];
					v.normal[1] = attrib.vertices[shapes[i].mesh.indices[j].normal_index * 3 - 2];
					v.normal[2] = attrib.vertices[shapes[i].mesh.indices[j].normal_index * 3 - 1];
				}
				else if (triangleIndex == 2)
				{
					v.normal[0] = attrib.vertices[shapes[i].mesh.indices[j].vertex_index * 3 - 6];
					v.normal[1] = attrib.vertices[shapes[i].mesh.indices[j].vertex_index * 3 - 5];
					v.normal[2] = attrib.vertices[shapes[i].mesh.indices[j].vertex_index * 3 - 4];
				}
			}
			else
			{
				memcpy((void*)& v.normal, (void*)& attrib.normals[(shapes[i].mesh.indices[j].normal_index * 3)], sizeof(float) * 3);
			}

			if ((shapes[i].mesh.indices[j].texcoord_index * 2) >= attrib.texcoords.size())
			{
				std::cerr << "Unable to put texcoord in " << shapes[i].name << std::endl;
				// fill with 0 
				v.textureCoordinate[0] = 0.f;
				v.textureCoordinate[1] = 0.f;
			}
			else
			{
				tinyobj::mesh_t* m = &shapes[i].mesh;
				v.textureCoordinate[0] = attrib.texcoords[(int)m->indices[j].texcoord_index * 2];
				v.textureCoordinate[1] = 1 - attrib.texcoords[(int)m->indices[j].texcoord_index * 2 + 1]; // Account for wavefront to opengl coordinate system conversion
			}


			mVertexData.push_back(v);
			if (j == shapes[i].mesh.indices.size() - 1)
			{
				SceneNode sceneNode;
				sceneNode.name = shapes[i].name.c_str();
				sceneNode.material = materials[materialId].name.c_str();
				sceneNode.vertexDataSize = mVertexData.size();
				sceneNode.vertexData = new Vertex[sceneNode.vertexDataSize];
				memcpy((void*)sceneNode.vertexData, (void*)mVertexData.data(), sizeof(Vertex) * sceneNode.vertexDataSize);
				sceneNode.startPosition = startPosition;
				sceneNode.endPosition = (unsigned)(sceneNode.startPosition + sceneNode.vertexDataSize);
				startPosition += (unsigned)sceneNode.vertexDataSize;
				sceneNode.primativeMode = 6;//GL_TRIANGLES;
				sceneNode.diffuseTextureId = 0;
				sceneNode.modelViewMatrix = matrix;
				addSceneNode(&sceneNode);
			}
		}
	}
}

void SceneBuilder::buildScene()
{
	if(sceneNodes.size() == 0)
	{
		std::cerr << " unable to build database" << std::endl;
		exit(-3);
	}


	for(size_t i=0; i<sceneNodes.size(); i++)
	{
		for(size_t j=0; j<sceneNodes[i].vertexDataSize; j++)
		{
			Vertex v;
			memcpy((void*) &v, (void*) &sceneNodes[i].vertexData[j], sizeof(Vertex));
			vertexData.push_back(v);
			indices.push_back((unsigned) indices.size());
		}
	}

	for(size_t i=0; i <sceneNodes.size(); i++)
	{
		if(materials.find(sceneNodes[i].material) == materials.end()  )
		{
			std::cerr << "Material " << sceneNodes[i].material << " was not loaded" << std::endl;
		}
		else
		{
			if(strlen(materials[sceneNodes[i].material.c_str()].diffuseTexName) > 0)
			{

				/* Check for paths that are not valid (unix support for paths with \ instead of / */
#ifndef _WIN32

				for(unsigned j=0; j<strlen(materials[sceneNodes[i].material.c_str()].diffuseTexName); j++)
				{
					if(materials[sceneNodes[i].material.c_str()].diffuseTexName[j] == '\\')
					{
						materials[sceneNodes[i].material.c_str()].diffuseTexName[j] = '/';
					}
				}
#endif

				addTexture(materials[sceneNodes[i].material.c_str()].diffuseTexName, sceneNodes[i].diffuseTextureId);
			}
		}
	}

	//Calculate Bounding Sphere radius
	for(int i=0; i<sceneNodes.size(); i++)
	{
		float center_x = 0.f, center_y = 0.f, center_z = 0.f;
		float r = 0.f;

		size_t vertexDataSize = sceneNodes[i].vertexDataSize;
		//Calculate local origin
		for(int j=0; j<vertexDataSize; j++)
		{
			center_x += sceneNodes[i].vertexData[j].vertex[0];
			center_y += sceneNodes[i].vertexData[j].vertex[1];
			center_z += sceneNodes[i].vertexData[j].vertex[2];
		}

		center_x /= (float)vertexDataSize;
		center_y /= (float)vertexDataSize;
		center_z /= (float)vertexDataSize;

		sceneNodes[i].center_x = center_x;
		sceneNodes[i].center_y = center_y;
		sceneNodes[i].center_z = center_z;

		for(int j=0; j<sceneNodes[i].vertexDataSize; j++)
		{
			float x = sceneNodes[i].vertexData[j].vertex[0];
			float y = sceneNodes[i].vertexData[j].vertex[1];
			float z = sceneNodes[i].vertexData[j].vertex[2];

			double nx = x - center_x;
			double ny = y - center_y;
			double nz = z - center_z;

			float r2 = (float) sqrt(nx*nx + ny*ny + nz*nz);

			if(r2 > r)
			{
				r = r2;
			}
		}
		if(r == 0)
		{
			std::cerr << "Warning, bounding sphere radius = 0 for " << sceneNodes[i].name << std::endl;
			r = 0.1f;
		}
		sceneNodes[i].radius = r;
	}
}

string intToStr(int i)
{
	char buffer[33];
	snprintf(buffer, sizeof(buffer), "%i", i);
	string s(buffer);
	return s;
}

string fToStr(float& f)
{
	char buffer[33];
	snprintf(buffer, sizeof(buffer), "%f", f);
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
		cerr << "SQL Error: " << sqlite3_errmsg(db) << endl;
	}

	for(size_t i=0; i<vertexData.size(); i++)
	{
		string vertexInsertSQL = "INSERT INTO vertex(px, py, pz, nx, ny, nz, tu, tv) VALUES (";
		vertexInsertSQL += fToStr(vertexData.at(i).vertex[0]);
		vertexInsertSQL += ",";
		vertexInsertSQL += fToStr(vertexData.at(i).vertex[1]);
		vertexInsertSQL += ",";
		vertexInsertSQL += fToStr(vertexData.at(i).vertex[2]);
		vertexInsertSQL += ",";
		vertexInsertSQL += fToStr(vertexData.at(i).normal[0]);
		vertexInsertSQL += ",";
		vertexInsertSQL += fToStr(vertexData.at(i).normal[1]);
		vertexInsertSQL += ",";
		vertexInsertSQL += fToStr(vertexData.at(i).normal[2]);
		vertexInsertSQL += ",";
		vertexInsertSQL += fToStr(vertexData.at(i).textureCoordinate[0]);
		vertexInsertSQL += ",";
		vertexInsertSQL += fToStr(vertexData.at(i).textureCoordinate[1]);
		vertexInsertSQL += ");";
		rc = sqlite3_exec(db, vertexInsertSQL.c_str(), 0, 0, &error_msg);
		if (rc != SQLITE_OK)
		{
			cerr << "SQL Error: " << sqlite3_errmsg(db) << endl;
		}
	}

	for(size_t i=0; i<sceneNodes.size(); i++)
	{
		string sceneNodeInsertSQL = "INSERT INTO scene_node(name, material_id, start_position, end_position, radius, center_x, center_y, center_z) VALUES (";
		sceneNodeInsertSQL += "'" + sceneNodes.at(i).name + "'";
		sceneNodeInsertSQL += ",";
		sceneNodeInsertSQL += intToStr(getMaterialId(sceneNodes.at(i).material));
		sceneNodeInsertSQL += ",";
		sceneNodeInsertSQL += intToStr(sceneNodes.at(i).startPosition);
		sceneNodeInsertSQL += ",";
		sceneNodeInsertSQL += intToStr(sceneNodes.at(i).endPosition);
		sceneNodeInsertSQL += ",";
		sceneNodeInsertSQL += fToStr(sceneNodes.at(i).radius);
		sceneNodeInsertSQL += ",";
		sceneNodeInsertSQL += fToStr(sceneNodes.at(i).center_x);
		sceneNodeInsertSQL += ",";
		sceneNodeInsertSQL += fToStr(sceneNodes.at(i).center_y);
		sceneNodeInsertSQL += ",";
		sceneNodeInsertSQL += fToStr(sceneNodes.at(i).center_z);
		sceneNodeInsertSQL += ");";
		rc = sqlite3_exec(db, sceneNodeInsertSQL.c_str(), 0, 0, &error_msg);
		if (rc != SQLITE_OK)
		{
			cerr << "SQL Error: " << sqlite3_errmsg(db) << endl;
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
			cerr << "SQL Error: " << sqlite3_errmsg(db) << endl;
		}
	}

	// Insert textures
	for (std::map<string,int>::iterator it=textures.begin(); it!=textures.end(); ++it)
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
		fileName += it->first;
		std::vector<unsigned char> imgContents = readFile(fileName.c_str());
		const unsigned char *zBlob = imgContents.data();
		int nBlob = (int) imgContents.size();
		sqlite3_bind_text(stmt, 1, it->first.c_str(), -1, SQLITE_STATIC);
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
		cerr << "SQL Error: " << sqlite3_errmsg(db) << endl;
	}

	cout << "Done writing database" << endl;
}

void _obj2sqlite(const char* wavefront, const char* database) {
	SceneBuilder sceneBuilder;
	sceneBuilder.addWavefront(wavefront, glm::translate(glm::mat4(1.f), glm::vec3(0.0, 0.0, 0.0)));
	sceneBuilder.buildScene();
	sceneBuilder.saveToDB(database);
}
