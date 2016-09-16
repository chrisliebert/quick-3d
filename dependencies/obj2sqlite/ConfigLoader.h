// Copyright (C) 2016 Chris Liebert

#ifndef _CONFIG_LOADER_H_
#define _CONFIG_LOADER_H_

#include <algorithm>
#include <cstring>
#include <istream>
#include <fstream>
#include <iostream>
#include <map>
#include <sstream>
#include <string>

// Cross-platform directory separator
#ifdef _WIN32
#define DIRECTORY_SEPARATOR "\\"
#else
#define DIRECTORY_SEPARATOR "/"
#endif

// Load configuration variables
class ConfigLoader
{
protected:
	const char* filename;
	std::map<std::string, std::string> vars;
public:
	// Load a .cfg file
	ConfigLoader(const char*);
	~ConfigLoader();
	bool getBool(const char*);
	bool getBool(std::string&);
	int getInt(const char*);
	int getInt(std::string&);
	float getFloat(const char*);
	float getFloat(std::string&);
	std::string& getVar(const char*);
	std::string& getVar(std::string&);
	bool hasVar(const char*);
	bool hasVar(std::string&);
	friend std::ostream& operator<<(std::ostream& os, ConfigLoader& dt); // used for debugging
	friend std::ostream& operator<<(std::ostream& os, ConfigLoader* dt); // used for debugging
};

#endif
