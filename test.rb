require_relative '../adventofcode-common/test'

test_and_exit { |daypad|
  rust_file = Dir.glob("#{__dir__}/src/bin/#{daypad}*.rs")
  next if rust_file.empty?
  raise "Need exactly one file not #{rust_file}" if rust_file.size != 1
  "#{__dir__}/target/release/#{File.basename(rust_file[0], '.rs')}"
}
