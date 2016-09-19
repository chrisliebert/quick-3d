// Copyright (C) 2016 Chris Liebert

#ifndef _SCENE_BUILDER_H_
#define _SCENE_BUILDER_H_

#include <fstream>
#include <iostream>
#include <map>
#include <string>
#include <vector>
#include <stdlib.h>
#include <stdio.h>

#include <glm/vec2.hpp>
#include <glm/vec3.hpp>
#include <glm/matrix.hpp>
#include <glm/mat4x4.hpp>
#include <glm/gtc/type_ptr.hpp>
#include <glm/gtc/matrix_transform.hpp>

#include "ConfigLoader.h"
#include "tiny_obj_loader.h"

// MSVC uses strcpy_s instead of strcpy
#ifdef _MSC_VER
#define strcpy(A, B) strcpy_s(A, B)
#endif

using std::cout;
using std::cerr;
using std::endl;
using std::string;

typedef struct
{
    float vertex[3];
    float normal[3];
    float textureCoordinate[2];
} Vertex;

typedef struct
{
    // Material name
    char name[64];
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
    char ambientTexName[64];
    char diffuseTexName[64];
    char specularTexName[64];
    char normalTexName[64];
} Material;


typedef struct SceneNode
{
    std::string name;
    std::string material;
    Vertex* vertexData;
    size_t vertexDataSize;
    glm::mat4 modelViewMatrix;
    unsigned startPosition;
    unsigned endPosition;
    int primativeMode;
    unsigned ambientTextureId;
    unsigned diffuseTextureId;
    unsigned normalTextureId;
    unsigned specularTextureId;
    float radius;
    float center_x, center_y, center_z; // position of center
} SceneNode;


class SceneBuilder
{
public:
    SceneBuilder();
    ~SceneBuilder();
    void addMaterial(Material*);
    void addSceneNode(SceneNode*);
    void addTexture(const char*, unsigned&);
    void addWavefront(const char*, glm::mat4);
    void buildScene();
    void saveToDB(const char*);
    int getMaterialId(string&);
private:
    unsigned startPosition;
    glm::mat4 modelViewProjectionMatrix;
    std::vector<Vertex> vertexData;
    std::vector<SceneNode> sceneNodes;
    std::vector<unsigned> indices;
    std::map<string, Material> materials;
    std::map<std::string, int> textures;
    ConfigLoader* cfg;
};

extern "C" void wavefrontToSQLite(const char* wavefront, const char* database);

#endif //_SCENE_BUILDER_H_
