call ./build.bat

git stash

git branch -D web
git checkout --orphan web
git reset --hard
git add out/*
git commit -m"update web"
git push -u origin web --force

git checkout main
git stash pop
