git branch -D web
git checkout --orphan web
rmdir out /S /Q
call ./build.bat
rmdir docs /S /Q
ren out docs
git reset
git add docs/*
git commit -m"update web"
git push -u origin web --force

git checkout main -f
