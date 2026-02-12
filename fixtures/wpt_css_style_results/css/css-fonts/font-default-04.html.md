# css/css-fonts/font-default-04.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-default-04.html"
}
```

## style[0]

```css

  @font-face {
    font-family: fwf;
    src: url(support/fonts/FontWithFancyFeatures.otf);
  }
  .tests {
	  font-family: fwf;
	  font-size: 4em;
	  line-height: 1.1;
    writing-mode: vertical-rl;
    text-orientation: upright;
    color: green;
  }
  .default {
    color: purple;
  }
  .test1 {
    font-feature-settings: "vert" on;
  }
  .test2 {
    font-feature-settings: "vert" 2;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
