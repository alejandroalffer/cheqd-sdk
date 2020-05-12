#!/usr/bin/env python
import time
import os
import sys

TIME_OFF_SET = 1515500000

def mkdirs(path):
    """This is like mkdir -p"""
    if os.path.isdir(path):
        return True
    try:
        if not os.path.isdir(path):
            os.makedirs(path)
        return True
    except FileExistsError as e:
        if os.access(path, os.W_OK):
            return True
        print("Path {}: exists but is unwritable".format(path))
        return False
    except OSError as e:
        if e.errno == 17: #This is fileexists
            return True
        print("Mkdir failed on: '{}'. Got error: {}".format(path, e.strerror))
        return False

def main():
    cache_build_ts_path = os.environ['CI_PROJECT_DIR'] + '/cache/build_ts'
    try:
        if os.path.exists(cache_build_ts_path):
            with open(cache_build_ts_path, 'r') as f:
                build_num = f.read()
        else:
            build_num = str(int(time.time() - TIME_OFF_SET))
            mkdirs(os.path.dirname(cache_build_ts_path))
            with open(cache_build_ts_path, 'w') as f:
                f.write(build_num)
    except IOError:
        exc_type, exc_value = sys.exc_info()[:2]
        exc_str = "{}: {}".format(exc_type.__name__, exc_value)
        print("WARNING!! Could not cache generated build num to: '{}': {}".format(cache_build_ts_path, exc_str))
    print(build_num)
    return build_num

if __name__ == '__main__':
    main()
