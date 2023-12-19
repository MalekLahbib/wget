# wget

This project is a student project done during the Zone01 Rouen cursus.
This project objective consists on recreating some functionalities of wget using a compiled language of your choice (like C, Rust, Go or other).

These functionalities will consist in:

    The normal usage of wget: downloading a file given an URL, example: wget https://some_url.ogr/file.zip
    Downloading a single file and saving it under a different name
    Downloading and saving the file in a specific directory
    Set the download speed, limiting the rate speed of a download
    Downloading a file in background
    Downloading multiple files at the same time, by reading a file containing multiple download links asynchronously
    Main feature will be to download an entire website, mirroring a website1- 

1-  The flag -B should be handled, this flag should download a file immediately to the background and the output should be redirected to a log file. When the program containing this flag is executed it should output : Output will be written to "wget-log".
2-  Download a file and save it under a different name by using the flag -O followed by the name you wish to save the file.
3-  It should also handle the path to where your file is going to be saved using the flag -P followed by the path to where you want to save the file.
4-  The program should handle speed limit. Basically the program can control the speed of the download by using the flag --rate-limit. If you download a huge file you can limit the speed of your download, preventing the program from using the full possible bandwidth of your connection.
5-  Downloading different files should be possible. For this the program will receive the -i flag followed by a file name that will contain all links that are to be downloaded.
6-  Mirror a website. This option should download the entire website being possible to use "part" of the website offline and for other useful reasons. For this you will have to download the website file system and save it into a folder that will have the domain name. Example: http://www.example.com, will be stored in a folder with the name www.example.com containing every file from the mirrored website. The flag should be --mirror.

The default usage of the flag will be to retrieve and parse the HTML or CSS from the given URL. This way retrieving the files that the document refers through tags. The tags that will be used for this retrieval must be a, link and img that contains attributes href and src.

You will have to implement some optional flags to go along with the --mirror flag.

Those flags will work based on Follow links. The command wget has several mechanisms that allows you to fine-tune which links it will follow. For This project you will have to implement the behavior of (note that this flags will be used in conjunction with the --mirror flag):

    Types of Files (--reject short hand -R)

    this flag will have a list of file suffixes that the program will avoid downloading during the retrieval

example:

$ go run . --mirror -R=jpg,gif https://example.com

    Directory-Based Limits (--exclude short hand -X)

    this flag will have a list of paths that the program will avoid to follow and retrieve. So if the URL is https://example.com and the directories are /js, /css and /assets you can avoid any path by using -X=/js,/assets. The fs will now just have /css.


I found a solution to the rate-limit flag but I wasn't very satisfied so it's commented. So for the moment I'm cheating by executing the wget command instead inside a function that I call. I'm open for any solution that will be near to the original wget function's way of working.
