// Copyright (C) 2016 Chris Liebert

#include "ConfigLoader.h"

void parseLine(std::string& line, std::map<std::string, std::string>& vars)
{
	const char* l = line.c_str();
	bool foundEquals = false;
	std::string lhs, rhs;

	// basic syntax check
	if(l[0] == '=') {
		std::cerr << "Invalid config line: " << line << std::endl;
		return;
	}

	for(int i=0; i<line.length(); i++) {
		char c = l[i];
		if(c == '#') break;
		if(c == '=') {
			foundEquals = true;
			continue;
		}
		if(c != ' ' && c != '\t') {
			if(!foundEquals) {
				// save the lhs
				lhs += c;
			} else {
				// save the rhs
				rhs += c;
			}
		}
	}

	if(lhs.length() > 0) vars.insert(std::make_pair(lhs, rhs));
}

ConfigLoader::ConfigLoader(const char* _filename) {
	filename = _filename;
	std::string filePath(filename);
	std::ifstream fileStream(filePath.c_str());
	if (!fileStream.is_open())
	{
		std::cerr << "Unable to load " << _filename  << ", writing default configuration" << std::endl;
		std::ofstream new_file(filePath.c_str(), std::fstream::app);
		new_file
		<< "model.directory = ." << std::endl
		<< "texture.directory = ." << std::endl;
		new_file.close();
		fileStream.open(filePath.c_str());
		if (!fileStream.is_open()) {
			std::cerr << "Failed to re-open " << filename << " after writing" << std::endl;
			exit(-4);
		}
	}

	std::string line;
	while (std::getline(fileStream, line))
	{
		int pos;
		line.erase(std::remove(line.begin(), line.end(), '\r'), line.end());
		parseLine(line, vars);
	}
	fileStream.close();
	std::cout << "Loaded " << filePath << std::endl;
}

ConfigLoader::~ConfigLoader() {
}

float ConfigLoader::getFloat(const char* var) {
	std::string v(var);
	return getFloat(v);
}

bool ConfigLoader::getBool(const char* var) {
	std::string v(var);
	return getBool(v);
}

bool ConfigLoader::getBool(std::string& var) {
	std::map<std::string, std::string>::iterator i = vars.find(var);
	if(i == vars.end()) {
		std::cerr << "Unable to load " << var << " from " << filename;
		exit(-3);
	}
	std::string s = i->second;
	if (s.compare("true") == 0 || s.compare("True") == 0) {
		return true;
	}
	else if (s.compare("false") == 0 || s.compare("False") == 0) {
		return false;
	}

	std::stringstream ss(s);
	bool val;

	while (ss >> val || !ss.eof()) {
		if (ss.fail()) {
			ss.clear();
			std::cerr << "Unable to parse variable " << var << " of " << s << " as bool." << std::endl;
			exit(6);
		}
	}

	return val;
}

float ConfigLoader::getFloat(std::string& var) {
	std::map<std::string, std::string>::iterator i = vars.find(var);
	if(i == vars.end()) {
		std::cerr << "Unable to load " << var << " from " << filename << std::endl;
		exit(-3);
	}
	std::string s = i->second;
	std::stringstream ss(s);
	float val;

	while(ss >> val || !ss.eof()) {
		if(ss.fail()) {
			ss.clear();
			std::cerr << "Unable to parse variable " << var << " of " << s << " as float." << std::endl;
			exit(6);
		}
	}

	return val;
}

int ConfigLoader::getInt(const char* var) {
	std::string v(var);
	return getInt(v);
}

int ConfigLoader::getInt(std::string& var) {
	std::map<std::string, std::string>::iterator i = vars.find(var);
	if(i == vars.end()) {
		std::cerr << "Unable to load " << var << " from " << filename << std::endl;
		exit(-3);
	}
	std::string s = i->second;
	std::stringstream ss(s);
	int val;

	while(ss >> val || !ss.eof()) {
		if(ss.fail()) {
			ss.clear();
			std::cerr << "Unable to parse variable " << var << " of " << s << " as integer." << std::endl;
			exit(6);
		}
	}

	return val;
}

std::string& ConfigLoader::getVar(const char* var)
{
	std::string v(var);
	return getVar(v);
}

std::string& ConfigLoader::getVar(std::string& var)
{
	std::map<std::string, std::string>::iterator i = vars.find(var);
	if(i == vars.end()) {
		std::cerr << "Unable to load " << var << " from " << filename << std::endl;
		exit(-3);
	}
	return i->second;
}

bool ConfigLoader::hasVar(const char* var)
{
	std::string s(var);
	return hasVar(s);
}

bool ConfigLoader::hasVar(std::string& var) {
	return vars.find(var) != vars.end();
}

std::ostream& operator<<(std::ostream& os, ConfigLoader& dt)
{
	os << "|------- " << dt.filename << " -------|" << std::endl;
	std::map<std::string, std::string>::iterator it;
	for (it=dt.vars.begin(); it!=dt.vars.end(); ++it) {
		os << it->first << " <- " << it->second << '\n';
	}
	os << "|--------";
	for(unsigned i=0; i<strlen(dt.filename); i++) {
		os << "-";
	}
	os << "--------|" << std::endl;;
	return os;
}

std::ostream& operator<<(std::ostream& os, ConfigLoader* dt)
{
	// dereference pointer and call function with reference
	os << *dt;
	return os;
}
