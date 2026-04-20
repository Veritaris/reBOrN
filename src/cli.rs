use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct RebornCliArgs {
    #[arg(
        long,
        short,
        action = clap::ArgAction::Count,
        help = "Set verbosity level, possible values: 0, 1, 2, 3. Higher values have same \
effect as 3\n"
    )]
    pub verbose: u8,

    #[arg(
        long,
        default_value = "false",
        help = "Enable debug output\n"
    )]
    pub debug: bool,

    #[arg(
        long,
        default_value = "false",
        help = "Print class after reading if --debug\n"
    )]
    pub print_class: bool,

    #[arg(
        long,
        default_value = "false",
        help = "Print constant pool after reading if --debug\n"
    )]
    pub print_cpool: bool,

    #[arg(
        long,
        default_value = "false",
        help = "Print code of function after reading if --debug\n"
    )]
    pub print_code: bool,

    #[arg(
        long,
        short,
        default_value = "1.7.10",
        help = "Game version to resolve default mappings path\n"
    )]
    pub game_version: String,

    #[arg(
        long,
        short,
        default_value = "stable_12",
        help = "Mappings 'channel', e.g. extra path after version\n"
    )]
    pub mappings: String,

    #[arg(
        long,
        short,
        help = "Extra mappings to be used. They are added over default mappings for chosen version
and may override already using mappings.
General extra mappings example: --extra-mappings='<target>:<source one>;<target>:<source two>'
Possible targets: 'fields', 'methods' and 'params'
Possible sources:
  - WebFile. Source is treated as web file source if it starts with 'http://' or 'https://'.
    Example: --extra-mappings='fields:https://example.org/fields.csv'

  - LocalFile. Source is treated if local file source if it starts 'file://' or found in filesystem
    Example: --extra-mappings='fields:file:///mnt/dev/fields.csv' OR
             --extra-mappings='fields:file://C:\\Users\\Me\\fields.csv' OR
             --extra-mappings='fields:C:\\Users\\Me\\fields.csv' OR

  - Inline. Inline mappings allow you to pass mappings just in CLI. Source is treated as inline if
           it is not treated as WebFile or LocalFile
    Example: --extra-mappings='fields:fieldOne=anotherFieldOne;methodOne=anotherMethodOne'
    Note: because mappings (fields, methods and params) are merged together you can pass all inline
          mappings with fields / methods / params target\n"
    )]
    pub extra_mappings: Vec<String>,

    #[arg(
        long,
        default_value = "false",
        help = "Cache or not WebFile mappings on you machine. \
        Also see --cache-dir and --set-cached-dir options\n"
    )]
    pub cache_web_files: bool,

    #[arg(
        long,
        default_value = "false",
        help = "Use given path as cache dir. Does not override saved cache dir\n"
    )]
    pub cache_dir: bool,

    #[arg(
        long,
        default_value = "false",
        help = "Set cache dir to provided path\n"
    )]
    pub set_cache_dir: String,

    #[arg(
        long,
        help = "Clean cache dir\n"
    )]
    pub clean_cache_dir: bool,

    #[arg(
        long,
        help = "Set filter for packages to deobf. If no values are given treated as deobf all \
classfiles.
Example: --packages-filter='com.example.packageOne;com.example.packageTwo'\n"
    )]
    pub packages_filter: Vec<String>,

    #[cfg(feature = "gui")]
    #[arg(
        long,
        short,
        help = "Input files to debf. You can pass both files and directories. If directory is \
passed then all jars will be found recursively and added to deobf list\
Example: --input=jars
         --input=some-mod-to-deobf.jar
"
    )]
    pub input: Vec<String>,

    #[cfg(not(feature = "gui"))]
    #[arg(
        long,
        short,
        required = true,
        help = "Input files to debf. You can pass both files and directories. If directory is \
passed then all jars will be found recursively and added to deobf list\
Example: --input=jars
         --input=some-mod-to-deobf.jar
"
    )]
    pub input: Vec<String>,

    #[arg(
        long,
        short,
        help = "Set output path for deobfuscated file. If omitted directory where command was \
called is used. If several output directories passed then the following logic is used:
 - num of input files equals to num of output dirs. Thus input jar will be written to the according
   directory
   Example: reBOrN --input=jars/a.jar,jars/b.jar --output=./a,./b
            reBOrN --input=jars/a.jar --output=/some/dir

 - num if input files not equals to num of output dirs. In that case file will be written into dir
   where original .jar file stored.
   Example: reBOrN --input=jars/a.jar,jars/b.jar --output=/some/dir
"
    )]
    pub output: Option<Vec<String>>,

    #[arg(
        long,
        default_value = "false",
        help = "Do not deobfuscate mod. This can be useful with high --deflate-compress-level
option if you want to compress\n"
    )]
    pub no_deobf: bool,

    #[arg(
        long,
        default_value = "false",
        help = "Show deobf progress\n"
    )]
    pub progress: bool,

    #[arg(
        long,
        default_value = "false",
        help = "Strip any resources but access transformers (that ens with _at.cfg) and MANIFEST.mf
useful for develop purposes to reduce dependency size\n"
    )]
    pub strip_resources: bool,

    #[arg(
        long,
        default_value = "2",
        help = "Set compression level for .class files. Classfiles compressed using Deflated
algorithm. 0..9 are faster but less compression is used, 10..256 have higher
compression rate but speed is dramatically reduced (expect 10 times slower remap!). To compress
resources also use --compress-resources flag, but this also affects processing duration\n"
    )]
    pub deflate_compress_level: i64,

    #[arg(
        long,
        default_value = "false",
        help = "Compress resources, not only classfiles\n"
    )]
    pub compress_resources: bool,
}
