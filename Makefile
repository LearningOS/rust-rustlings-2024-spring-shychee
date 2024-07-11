# make push msg="xxx"
push:
	git add -A
	git config --global user.email "shychee96@gmail.com"
	git config --global user.name "shychee"

	git commit -m '$(msg)'

	git pull --rebase
	git push --set-upstream origin HEAD