# css/css-variables/url-syntax-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/url-syntax-crash.html"
}
```

## style[0]

```css

@property --my-url {
  syntax: "<url> | none";
  inherits: true;
  initial-value: none;
}
:root {
  --my-url: url(blah);
}

* {
  --foo: var(--my-url);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
