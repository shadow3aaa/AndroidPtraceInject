# Android Ptrace Inject

![](https://img.shields.io/badge/Android-Build-green)
![](https://img.shields.io/badge/Android%204~12-Support-green)
![](https://img.shields.io/badge/arm64--v8a-Support-green)
![](https://img.shields.io/badge/x86-Support-green)

> 中文可以参考我的注释内容进行理解 <br>
> 我写的注释相对来说比较全面了

## How to build

* just call cargo build

## How to use

```shell
# Here are the parameters of the Inject command line tool:
#   Some parameters are optional.
#   -p process 's pid <-- optional
#   -n process 's package name <-- optional
#   -f whether to start App <-- optional
#   ---- //  /data/local/tmp/Inject -f -n XXX <-- error
#   ---- //  cd /data/local/tmp && ./Inject -f -n XXX <-- right
#   -so so path for injection <-- mandatory
#   -symbols specify a symbol in so <-- optional
# For examples:
cd /data/local/tmp && ./Inject -f -n bin.mt.plus -so /data/local/tmp/libHook.so -symbols hello
```

# TODO LIST

## Finished

- [x] First Inject Succeeded

- [x] Handle Parameter

- [x] Handle SELinux

- [x] Handle Libs

- [x] Succeed Inject for arm64-v8a

- [x] Succeed Inject for Android 9 and Android 11

- [x] Adapt to all Android versions

- [x] Succeed Inject for x86

## Future

- [ ] Fix bugs for armeabi-v7a

- [ ] Adapt to the ABIs of each device, such as armeabi-v7a and x86_64

# Credits

[Adrill](https://github.com/mustime/Adrill) By mustime

[SharkInject](https://github.com/bigGreenPeople/SharkInject) By bigGreenPeople

[androidinject](https://github.com/mergerly/androidinject) By mergerly

[TinyInjector](https://github.com/shunix/TinyInjector) By shunix
