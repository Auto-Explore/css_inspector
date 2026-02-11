# css/css-fonts/font-feature-resolution-002.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-feature-resolution-002.html"
}
```

## style[0]

```css

  @font-face {
    font-family: lato-ffs-;
    src: url(/fonts/Lato-Medium-Liga.ttf);
  }
  @font-face {
    font-family: lato-ffs-0;
    src: url(/fonts/Lato-Medium-Liga.ttf);
    font-feature-settings: 'dlig' off;
  }
  @font-face {
    font-family: lato-ffs-1;
    src: url(/fonts/Lato-Medium-Liga.ttf);
    font-feature-settings: 'dlig' on;
  }
  .test, .ref {
    font-family: lato-ffs-;
    font-size: 2em;
    position: absolute;
  }
  .test { color: green; }
  .ref { color: red; }

  .align { width: 1em; }

  .ff-  { font-family: lato-ffs- ; }
  .ff-0 { font-family: lato-ffs-0; }
  .ff-1 { font-family: lato-ffs-1; }

  .fvl-  { }
  .fvl-n { font-variant-ligatures: normal; }
  .fvl-0 { font-variant-ligatures: no-discretionary-ligatures; }
  .fvl-1 { font-variant-ligatures: discretionary-ligatures; }

  .ls-  { }
  .ls-0 { letter-spacing: 0em; }
  .ls-1 { letter-spacing: 0.1em; }

  .ffs-  { }
  .ffs-n { font-feature-settings: normal; }
  .ffs-0 { font-feature-settings: 'dlig' off; }
  .ffs-1 { font-feature-settings: 'dlig' on; }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “font-feature-settings”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font-feature-settings”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
