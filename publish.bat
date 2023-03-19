call ./build.bat
git branch -D web
git checkout --orphan web
rmdir docs /S /Q
ren out docs
git add .
git commit -m"update web"
git push -u origin web --force

git checkout main -f
