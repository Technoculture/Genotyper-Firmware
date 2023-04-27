# Run gitpoll script
echo " ⏳Starting gitpoll script"
# Make sure all outputs to stdout have a tab at the beginning, all outputs should be on the same line
python -u ./config_gitpoll/gitpoll/gitpoll.py | sed 's/^/ ┣╼ /'
echo " ✅ Done"
