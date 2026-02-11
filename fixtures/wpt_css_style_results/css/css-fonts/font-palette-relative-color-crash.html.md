# css/css-fonts/font-palette-relative-color-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-palette-relative-color-crash.html"
}
```

## style[0]

```css

  @font-face {
    font-family: Foo;
    src: url(notfound.ttf);
  }
  @font-palette-values --foo {
    font-family: Foo;
    override-colors: 0 lch(from blue calc(0.5 * l) c h);
  }
  #target {
    font-family: Foo;
    font-palette: --foo;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “font-palette”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
