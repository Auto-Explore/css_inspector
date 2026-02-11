# css/css-color/at-color-profile-001.html

```json
{
  "format_version": 3,
  "file": "css/css-color/at-color-profile-001.html"
}
```

## style[0]

```css

    /* sRGB except red and green colorants are swapped */
    @color-profile --foo {
        src: url(support/swapped.icc);
    }
    .test { background-color: red; width: 12em; height: 6em; margin-top: 0; }
    .ref { background-color: #090; width: 12em; height: 6em; margin-bottom: 0; } /* red-green swap of #900 sRGB */
    .test { background-color: color(--foo 0.6 0 0); } /* hex 99 / FF is decimal 0.6 */
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
