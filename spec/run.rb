require 'rspec'

project_root = File.expand_path('..', __dir__)
$topaz = File.join(project_root, 'target', 'release', 'topaz')

def run_dir(dir)
	Dir.each_child(dir) do |filename|
		run_file File.join(dir, filename)
	end
end

def run_file(file)
	contents = File.open(file, 'r').read
	expects = []
	File.open(file).each_line do |line|
		match = read_expects(line)
		expects << match if !match.nil?
	end

	output = IO.popen("#{$topaz} #{file}", 'r+') do |pipe|
		pipe.close_write
		pipe.gets(nil)
	end

	it "runs tests in #{file}" do
		expect(output.split("\n")).to match_array expects
	end
end

def read_expects(line)
	expect_regex = /^\W*# expect: (.*)$/
	line.match(expect_regex) { |m| m[1] }
end

describe 'topaz test suite' do
	Dir.each_child(__dir__) do |filename|
		fullpath =  File.join __dir__, filename
		next if !File.directory? fullpath
	
		run_dir fullpath
	end
end
