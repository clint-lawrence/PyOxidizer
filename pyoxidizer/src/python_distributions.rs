// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Defines known Python distributions.

use {
    crate::py_packaging::distribution::{
        DistributionFlavor, PythonDistributionLocation, PythonDistributionRecord,
    },
    itertools::Itertools,
    once_cell::sync::Lazy,
};

pub struct PythonDistributionCollection {
    dists: Vec<PythonDistributionRecord>,
}

impl PythonDistributionCollection {
    /// Find a Python distribution given requirements.
    ///
    /// `target_triple` is the Rust machine triple the distribution is built for.
    /// `flavor` is the type of Python distribution.
    /// `python_major_minor_version` is an optional `X.Y` version string being
    /// requested. If `None`, `3.9` is assumed.
    pub fn find_distribution(
        &self,
        target_triple: &str,
        flavor: &DistributionFlavor,
        python_major_minor_version: Option<&str>,
    ) -> Option<PythonDistributionRecord> {
        let python_major_minor_version = python_major_minor_version.unwrap_or("3.9");

        self.dists
            .iter()
            .filter(|dist| dist.python_major_minor_version == python_major_minor_version)
            .filter(|dist| dist.target_triple == target_triple)
            .filter(|dist| match flavor {
                DistributionFlavor::Standalone => true,
                DistributionFlavor::StandaloneStatic => !dist.supports_prebuilt_extension_modules,
                DistributionFlavor::StandaloneDynamic => dist.supports_prebuilt_extension_modules,
            })
            .cloned()
            .next()
    }

    /// Obtain records for all registered distributions.
    #[allow(unused)]
    pub fn iter(&self) -> impl Iterator<Item = &PythonDistributionRecord> {
        self.dists.iter()
    }

    /// All target triples of distributions in this collection.
    #[allow(unused)]
    pub fn all_target_triples(&self) -> impl Iterator<Item = &str> {
        self.dists
            .iter()
            .map(|dist| dist.target_triple.as_str())
            .sorted()
            .dedup()
    }
}

pub static PYTHON_DISTRIBUTIONS: Lazy<PythonDistributionCollection> = Lazy::new(|| {
    let dists = vec![
        // Linux glibc linked.
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.8.13%2B20220502-x86_64-unknown-linux-gnu-pgo-full.tar.zst".to_string(),
                sha256: "ea71695a7c8c08064388c9eb8c612187c6b76748f1ab2c42f65ea946be275d98".to_string(),
            },
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.9.12%2B20220502-x86_64-unknown-linux-gnu-pgo-full.tar.zst".to_string(),
                sha256: "1d88b590599aa1d1589f226b23dab3f4491754fbc6ef5697e0a46d27be11ba1f".to_string(),
            },
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.9.12%2B20220502-x86_64_v2-unknown-linux-gnu-pgo-full.tar.zst".to_string(),
                sha256: "4e234820eb31079b2b5a0b729088fa0dce5310a544b732e565035661cea77b06".to_string(),
            },
            target_triple: "x86_64_v2-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.9.12%2B20220502-x86_64_v3-unknown-linux-gnu-pgo-full.tar.zst".to_string(),
                sha256: "7bfdd65b6c672d733e85ed70e6af61778504efb595ad70cb066d38af7c30188d".to_string(),
            },
            target_triple: "x86_64_v3-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.10.4%2B20220502-x86_64-unknown-linux-gnu-pgo-full.tar.zst".to_string(),
                sha256: "1822b690f971c4c9ccf3bc3b5393c4454c22fcb70403c8ae07cddff56cc32afd".to_string(),
            },
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.10.4%2B20220502-x86_64_v2-unknown-linux-gnu-pgo-full.tar.zst".to_string(),
                sha256: "ee5a0c4175a7df68b3c440f2c257b8b20aee569299b031d00ed1eda0a1df8d64".to_string(),
            },
            target_triple: "x86_64_v2-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.10.4%2B20220502-x86_64_v3-unknown-linux-gnu-pgo-full.tar.zst".to_string(),
                sha256: "cab3662b701c62a33553445670b459a3745bfe6f9152989750c9ce3dbc52fea1".to_string(),
            },
            target_triple: "x86_64_v3-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },

        // Linux musl.
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.8.13%2B20220502-x86_64-unknown-linux-musl-noopt-full.tar.zst".to_string(),
                sha256: "9aead69e67634623f3f3007ccade11359619721a96381244bcbdc3fa66001071".to_string(),
            },
            target_triple: "x86_64-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.9.12%2B20220502-x86_64-unknown-linux-musl-noopt-full.tar.zst".to_string(),
                sha256: "cc0967c864365b13deb0fc9ba5f4a4125bf320cd4a903c8a9bf78fd7d7f64ed4".to_string(),
            },
            target_triple: "x86_64-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.9.12%2B20220502-x86_64_v2-unknown-linux-musl-noopt-full.tar.zst".to_string(),
                sha256: "4013a0dad0a29f095fa4bc10136f9bdae2025d85e8b86ae5fabf5db7d2a3d9ff".to_string(),
            },
            target_triple: "x86_64_v2-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.9.12%2B20220502-x86_64_v3-unknown-linux-musl-noopt-full.tar.zst".to_string(),
                sha256: "de6e34557c8575235b3a75eb50b09d4ebd3dd5a3d04d382dfab62be27865c478".to_string(),
            },
            target_triple: "x86_64_v3-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.10.4%2B20220502-x86_64-unknown-linux-musl-noopt-full.tar.zst".to_string(),
                sha256: "c183a4752e3e55340222f5a5ba590de2b26d2b82dff5b94fd993f0be138c936c".to_string(),
            },
            target_triple: "x86_64-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.10.4%2B20220502-x86_64_v2-unknown-linux-musl-noopt-full.tar.zst".to_string(),
                sha256: "41a045c592ec8c4381f262ddd787645fb5322c8798cd711be96881e85b2cb008".to_string(),
            },
            target_triple: "x86_64_v2-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.10.4%2B20220502-x86_64_v3-unknown-linux-musl-noopt-full.tar.zst".to_string(),
                sha256: "b8d059a86169a8d260b0f670686851846c225ab221ff66dc8704116d80580aa6".to_string(),
            },
            target_triple: "x86_64_v3-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: true,
        },

        // The order here is important because we will choose the
        // first one. We prefer shared distributions on Windows because
        // they are more versatile: statically linked Windows distributions
        // don't declspec(dllexport) Python symbols and can't load shared
        // shared library Python extensions, making them a pain to work
        // with.

        // Windows shared.
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.8.13%2B20220502-i686-pc-windows-msvc-shared-pgo-full.tar.zst".to_string(),
                sha256: "3427a67585bd9f8ea88d27d5488ddb478a945ff7b5d75ba0d9a15d0c1fe195bf".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.9.12%2B20220502-i686-pc-windows-msvc-shared-pgo-full.tar.zst".to_string(),
                sha256: "361b8fa66d6b5d5623fd5e64af29cf220a693ba86d031bf7ce2b61e1ea50f568".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.10.4%2B20220502-i686-pc-windows-msvc-shared-pgo-full.tar.zst".to_string(),
                sha256: "6ef65c0e7aa91234acf86a423324d63a70c5b4c694cbd2947d358714497233c2".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.8.13%2B20220502-x86_64-pc-windows-msvc-shared-pgo-full.tar.zst".to_string(),
                sha256: "0048e0681ac83c9e57c4f5e457c1e06677edce3c2c9dd478353e5483bed983a4".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.9.12%2B20220502-x86_64-pc-windows-msvc-shared-pgo-full.tar.zst".to_string(),
                sha256: "c49f8b07e9c4dcfd7a5b55c131e882a4ebdf9f37fef1c7820c3ce9eb23bab8ab".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.10.4%2B20220502-x86_64-pc-windows-msvc-shared-pgo-full.tar.zst".to_string(),
                sha256: "37764a9a1683eb80d16de36e7fa9dd0e17d9d415dbc046893eb92d13bd03b1db".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },

        // Windows static.
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.8.13%2B20220502-i686-pc-windows-msvc-static-noopt-full.tar.zst".to_string(),
                sha256: "f1ac46fbd6726129df03adb6573d8f9f64652a61545bfdbfad6be93613479252".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.9.12%2B20220502-i686-pc-windows-msvc-static-noopt-full.tar.zst".to_string(),
                sha256: "10677ddd613e2cd62adff641b9dcdbeee05234cc84c662323ffc53b8215c8dd6".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.10.4%2B20220502-i686-pc-windows-msvc-static-noopt-full.tar.zst".to_string(),
                sha256: "f1268191d3de9870aa032c111e78412211e7cb3e42f03e1674d060fd082772e8".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.8.13%2B20220502-x86_64-pc-windows-msvc-static-noopt-full.tar.zst".to_string(),
                sha256: "bbedefb83dcae584ed6591ff4dfd6ed85ac9d5097484a233a6422879058e34d6".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.9.12%2B20220502-x86_64-pc-windows-msvc-static-noopt-full.tar.zst".to_string(),
                sha256: "39bb260122a0d7d97b2b88d86affd779f6e0cd71034ce16ba892636cdd451458".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.10.4%2B20220502-x86_64-pc-windows-msvc-static-noopt-full.tar.zst".to_string(),
                sha256: "1ac48a140e7c5c2f16017119046356e59515d34a8479bbf00256bd0573f871b0".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },

        // macOS.
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.9.12%2B20220502-aarch64-apple-darwin-pgo-full.tar.zst".to_string(),
                sha256: "5b20ea35650fc67b00e59871b114e831af3faa03a000187f3ac9e8e38456351a".to_string(),
            },
            target_triple: "aarch64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.10.4%2B20220502-aarch64-apple-darwin-pgo-full.tar.zst".to_string(),
                sha256: "748ddb0f28992837b5951a23e83ae81bc724fd9e750859f3aa0b2355fb030ea5".to_string(),
            },
            target_triple: "aarch64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.8.13%2B20220502-x86_64-apple-darwin-pgo-full.tar.zst".to_string(),
                sha256: "c9b7dc0003906589b4db96bde1c18ae6c12257b11b60026f1e8227f5f8bdb231".to_string(),
            },
            target_triple: "x86_64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.9.12%2B20220502-x86_64-apple-darwin-pgo-full.tar.zst".to_string(),
                sha256: "ef2865504cf53e2fae7f8a708cf4bea8ecef2e0964777cc1ea6c276bbc76ade3".to_string(),
            },
            target_triple: "x86_64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220502/cpython-3.10.4%2B20220502-x86_64-apple-darwin-pgo-full.tar.zst".to_string(),
                sha256: "b8468c6f9ff21acfafaf8068f08705e0f529db6f92c455bccd3612957bdc525e".to_string(),
            },
            target_triple: "x86_64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
    ];

    PythonDistributionCollection { dists }
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_target_triples() {
        assert_eq!(
            PYTHON_DISTRIBUTIONS
                .all_target_triples()
                .collect::<Vec<_>>(),
            vec![
                "aarch64-apple-darwin",
                "i686-pc-windows-msvc",
                "x86_64-apple-darwin",
                "x86_64-pc-windows-msvc",
                "x86_64-unknown-linux-gnu",
                "x86_64-unknown-linux-musl",
                "x86_64_v2-unknown-linux-gnu",
                "x86_64_v2-unknown-linux-musl",
                "x86_64_v3-unknown-linux-gnu",
                "x86_64_v3-unknown-linux-musl",
            ]
        );
    }
}
