// Copyright (C) 2016 Chris Liebert

#include "SceneBuilder.h"

int main(int argc, char** argv)
{
	if(argc < 2 || argc > 3)
	{
		cerr << "Usage: " << argv[0] << " objfile.obj [texture directory]" << endl;
		return -1;
	}
	if(strlen(argv[1]) < 5)
	{
		cerr << "Must end in .obj" << endl;
		return -2;
	}
	string inputFileName(argv[1]);
	string outFileName(inputFileName.substr(0, strlen(argv[1]) - 4) + ".db");
	wavefrontToSQLite(argv[1], outFileName.c_str());
	return 0;
}
