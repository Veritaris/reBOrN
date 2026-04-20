use crate::mappings::{DeobfMappingsType, ModLoader};
use clap::Parser;

#[derive(Parser, Clone, Debug)]
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

    #[arg(long, default_value = "false", help = "Enable debug output\n")]
    pub debug: bool,

    #[arg(long, default_value = "false", help = "Print class after reading if --debug\n")]
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
        value_enum,
        default_value_t = ModLoader::Forge,
        help = "Mod loader to deobfuscate jar for"
    )]
    pub mod_loader: ModLoader,

    #[arg(
        long,
        value_enum,
        default_value_t = DeobfMappingsType::VersionsJSON,
        help = "Deobfuscate jar versions source - versions.json or custom"
    )]
    pub mappings_type: DeobfMappingsType,

    #[arg(
        long,
        short,
        default_value = "stable",
        help = "Mappings 'channel', e.g. extra path after version. Commonly used are 'stable' for
old versions, 'official' and 'parchment' on modern versions of Forge\n"
    )]
    pub mappings_channel: String,

    #[arg(
        long,
        default_value = "12",
        help = "Mappings 'version', e.g. extra path after version. Commonly used are '12' for
stable channel on old versions\n"
    )]
    pub mappings_version: String,

    #[arg(
        long,
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
        help = "Show versions from versions.json and downloaded mappings"
    )]
    pub show_versions: bool,

    #[arg(
        long,
        default_value = "false",
        help = "Use given path as cache dir. Does not override saved cache dir\n"
    )]
    pub cache_dir: bool,

    #[arg(long, default_value = "false", help = "Set cache dir to provided path\n")]
    pub set_cache_dir: String,

    #[arg(long, help = "Clean cache dir\n")]
    pub clean_cache_dir: bool,

    #[arg(
        long,
        help = "Set filter for packages to deobf. If no values are given treated as deobf all \
classfiles.
Example: --packages-filter='com.example.packageOne;com.example.packageTwo'\n"
    )]
    pub packages_filter: Vec<String>,

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

    #[arg(long, default_value = "false", help = "Show deobf progress\n")]
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

    #[arg(long, default_value = "false", help = "Compress resources, not only classfiles\n")]
    pub compress_resources: bool,
}

impl RebornCliArgs {
    pub fn set_verbose(&mut self, verbose: u8) -> &mut Self {
        self.verbose = verbose;
        self
    }
    pub fn set_debug(&mut self, debug: bool) -> &mut Self {
        self.debug = debug;
        self
    }
    pub fn set_print_class(&mut self, print_class: bool) -> &mut Self {
        self.print_class = print_class;
        self
    }
    pub fn set_print_cpool(&mut self, print_cpool: bool) -> &mut Self {
        self.print_cpool = print_cpool;
        self
    }
    pub fn set_print_code(&mut self, print_code: bool) -> &mut Self {
        self.print_code = print_code;
        self
    }
    pub fn set_game_version(&mut self, game_version: String) -> &mut Self {
        self.game_version = game_version;
        self
    }
    pub fn set_mod_loader(&mut self, mod_loader: ModLoader) -> &mut Self {
        self.mod_loader = mod_loader;
        self
    }
    pub fn set_mappings_channel(&mut self, mappings_channel: String) -> &mut Self {
        self.mappings_channel = mappings_channel;
        self
    }
    pub fn set_mappings_version(&mut self, mappings_version: String) -> &mut Self {
        self.mappings_version = mappings_version;
        self
    }
    pub fn set_extra_mappings(&mut self, extra_mappings: Vec<String>) -> &mut Self {
        self.extra_mappings = extra_mappings;
        self
    }
    pub fn set_cache_web_files(&mut self, cache_web_files: bool) -> &mut Self {
        self.cache_web_files = cache_web_files;
        self
    }
    pub fn set_show_versions(&mut self, show_versions: bool) -> &mut Self {
        self.show_versions = show_versions;
        self
    }
    pub fn set_cache_dir(&mut self, cache_dir: bool) -> &mut Self {
        self.cache_dir = cache_dir;
        self
    }
    pub fn set_set_cache_dir(&mut self, set_cache_dir: String) -> &mut Self {
        self.set_cache_dir = set_cache_dir;
        self
    }
    pub fn set_clean_cache_dir(&mut self, clean_cache_dir: bool) -> &mut Self {
        self.clean_cache_dir = clean_cache_dir;
        self
    }
    pub fn set_packages_filter(&mut self, packages_filter: Vec<String>) -> &mut Self {
        self.packages_filter = packages_filter;
        self
    }
    pub fn set_input(&mut self, input: Vec<String>) -> &mut Self {
        self.input = input;
        self
    }
    pub fn set_output(&mut self, output: Option<Vec<String>>) -> &mut Self {
        self.output = output;
        self
    }
    pub fn set_no_deobf(&mut self, no_deobf: bool) -> &mut Self {
        self.no_deobf = no_deobf;
        self
    }
    pub fn set_progress(&mut self, progress: bool) -> &mut Self {
        self.progress = progress;
        self
    }
    pub fn set_strip_resources(&mut self, strip_resources: bool) -> &mut Self {
        self.strip_resources = strip_resources;
        self
    }
    pub fn set_deflate_compress_level(&mut self, deflate_compress_level: i64) -> &mut Self {
        self.deflate_compress_level = deflate_compress_level;
        self
    }

    #[inline(always)]
    pub fn get_deflate_compress_level(&self) -> Option<i64> {
        match self.deflate_compress_level {
            0 => None,
            x => Some(x),
        }
    }
    pub fn set_compress_resources(&mut self, compress_resources: bool) -> &mut Self {
        self.compress_resources = compress_resources;
        self
    }
}

impl Default for RebornCliArgs {
    fn default() -> Self {
        Self {
            verbose: 0,
            debug: false,
            print_class: false,
            print_cpool: false,
            print_code: false,
            game_version: "1.7.10".to_string(),
            mod_loader: ModLoader::Forge,
            mappings_type: DeobfMappingsType::VersionsJSON,
            mappings_channel: "stable".to_string(),
            mappings_version: "12".to_string(),
            extra_mappings: vec![],
            cache_web_files: false,
            show_versions: false,
            cache_dir: false,
            set_cache_dir: "".to_string(),
            clean_cache_dir: false,
            packages_filter: vec![],
            input: vec![],
            output: Some(vec![]),
            no_deobf: false,
            progress: false,
            strip_resources: false,
            deflate_compress_level: 2,
            compress_resources: false,
        }
    }
}
