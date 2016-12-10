// Copyright (C) 2016 Chris Liebert

#ifndef _SCENE_BUILDER_H_
#define _SCENE_BUILDER_H_

#include <fstream>
#include <iostream>
#include <map>
#include <sstream>
#include <string>
#include <vector>
#include <set>
#include <cstdlib>

#include <glm/vec2.hpp>
#include <glm/vec3.hpp>
#include <glm/matrix.hpp>
#include <glm/mat4x4.hpp>
#include <glm/gtc/type_ptr.hpp>
#include <glm/gtc/matrix_transform.hpp>

#include "ConfigLoader.h"
#include "tiny_obj_loader.h"

#ifdef _MSC_VER
#define strncpy(A,B,C) strncpy_s(A,B,C)
#endif

#define MAX_MATERIAL_NAME_LENGTH 128

using std::cout;
using std::cerr;
using std::endl;
using std::string;

typedef struct Vertex
{
    float position[3];
    float normal[3];
    float textureCoordinate[2];
} Vertex;

typedef struct Material
{
    // Material name
    char name[MAX_MATERIAL_NAME_LENGTH];
    float ambient[3];
    float diffuse[3];
    float specular[3];
    float transmittance[3];
    float emission[3];
    float shininess;
    float ior;      // index of refraction
    float dissolve; // 1 == opaque; 0 == fully transparent
    // illumination model (see http://www.fileformat.info/format/material/)
    int illum;
    //Texture file names
    char ambientTexName[MAX_MATERIAL_NAME_LENGTH];
    char diffuseTexName[MAX_MATERIAL_NAME_LENGTH];
    char specularTexName[MAX_MATERIAL_NAME_LENGTH];
    char normalTexName[MAX_MATERIAL_NAME_LENGTH];
} Material;


typedef struct SceneNode
{
    std::string name;
    int materialId;
    std::vector<Vertex> vertexData;
    glm::mat4 modelViewMatrix;
    unsigned startPosition;
    unsigned endPosition;
    float radius;		// max distance from object center
    float center[3];	// position of object center
} SceneNode;


class SceneBuilder
{
public:
    SceneBuilder();
    ~SceneBuilder();
    void addMaterial(Material*);
    void addSceneNode(SceneNode*);
    void addTexture(const char*);
    bool addWavefront(const char*, glm::mat4);
    void saveToDB(const char*);
    int getMaterialId(string&);
private:
    unsigned startPosition;
    std::vector<SceneNode> sceneNodes;
    std::map<string, Material> materials;
    std::set<std::string> textures;
    ConfigLoader* cfg;
};

extern "C" void wavefrontToSQLite(const char* wavefront, const char* database);

#endif //_SCENE_BUILDER_H_
