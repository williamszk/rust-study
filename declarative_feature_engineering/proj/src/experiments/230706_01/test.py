import yaml 
from yaml import CLoader as Loader

if __name__ == '__main__':

    stream = open("foo.yaml", 'r')
    dictionary = yaml.load(stream, Loader=Loader)
    for key, value in dictionary.items():
        print (key + " : " + str(value))
    # print(dictionary)