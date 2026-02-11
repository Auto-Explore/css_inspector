# css/css-values/attr-universal-selector.html

```json
{
  "format_version": 3,
  "file": "css/css-values/attr-universal-selector.html"
}
```

## style[0]

```css

  @property --some-string {
    syntax: "<string>";
    inherits: true;
    initial-value: "empty";
  }
  * {
    --some-string: "https://does-not-exist.test/404.png";
  }
  div {
    --some-string: attr(data-foo);
    background-image: image-set(var(--some-string));
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
