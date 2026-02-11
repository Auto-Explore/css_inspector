# css/css-fonts/font-synthesis-position-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-synthesis-position-001-ref.html"
}
```

## style[0]

```css

    /* Lato has superscript Latin letters, but not subscript ones;
     * digits are available in both super- and subscript form.
     */
    @font-face {
        font-family: "lato";
        src: url(/fonts/Lato-Medium.ttf);
    }
    .test {
        font-family: "lato";
        font-size: 2em;
    }
    .sub {
        font-feature-settings: 'subs' 1;
    }
    .super {
        font-feature-settings: 'sups' 1;
    }
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
