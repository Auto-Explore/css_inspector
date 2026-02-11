# css/css-values/attr-security.html

```json
{
  "format_version": 3,
  "file": "css/css-values/attr-security.html"
}
```

## style[0]

```css

  @property --some-string {
    syntax: "<string>";
    inherits: false;
    initial-value: "empty";
  }
  @property --some-string-list {
    syntax: "<string>+";
    inherits: false;
    initial-value: "empty";
  }
  div {
    --condition-val: 3;
    --str: text;
    --true: true;
    --some-string: attr(data-foo);
    --some-string-list: "https://does-not-exist2.test/404.png" attr(data-foo);
    --some-other-url: attr(data-foo);
    --image-set-valid:  url("https://does-not-exist.test/404.png") type(attr(data-foo));
    --image-set-invalid: attr(data-foo type(<url>)) 1x;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
