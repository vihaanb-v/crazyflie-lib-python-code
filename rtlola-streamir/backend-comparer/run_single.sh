cur_dir=$(pwd)
spec=$1
trace=$2
script_dir=$(dirname $0)
if [ -z "$spec" -o -z "$trace" ]; then
  echo "Give spec and trace as arguments"
  exit
fi
test_json="[\
  {\
	\"spec\": \"$cur_dir/$spec\",\
	\"traces\": [\
	  \"$cur_dir/$trace\"\
	]\
  }\
]"
test_file="$(mktemp)"
echo $test_json | jq > "$test_file"
$script_dir/run.sh $test_file
