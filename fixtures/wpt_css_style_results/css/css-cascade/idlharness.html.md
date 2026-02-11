# css/css-cascade/idlharness.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/idlharness.html"
}
```

## style[0]

```css

@layer bar, baz;
@import url('data:text/css,') layer(qux);
@layer foo { }
@scope (div) to (span) { }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
