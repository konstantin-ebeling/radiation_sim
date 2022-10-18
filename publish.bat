call ./build.bat

git stash

git branch -D web
git checkout --orphan web
rename out docs
git add docs/*
git commit -m"update web"
git push -u origin web --force

git checkout main
git stash pop
