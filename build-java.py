import os
import subprocess
import shutil

def copy_jar(path: str) -> None:
    cur_dir = os.getcwd()

    os.chdir(f'samples/{f}')
    print(f'Compiling {f}')
    subprocess.run(['mvn', 'package'])
    print(f'Compiled {f}')

    os.chdir(cur_dir)

    jar_path = f'samples/{f}.jar'
    shutil.copy(f'samples/{f}/target/{f}-1.0-SNAPSHOT.jar', jar_path)
    print(f'Copied {jar_path} over')

if __name__ == '__main__':
    dirs = os.listdir('samples')
    cur_dir = os.getcwd()
    for f in dirs:
        fpath = f'samples/{f}'
        if not os.path.isdir(fpath):
            print(f'Ignoring {fpath}')
            continue

        print(f'Building {f}')
        copy_jar(f)
