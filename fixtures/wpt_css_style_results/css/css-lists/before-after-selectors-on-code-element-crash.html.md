# css/css-lists/before-after-selectors-on-code-element-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/before-after-selectors-on-code-element-crash.html"
}
```

## style[0]

```css

    code:before {
      content: counter(dummy,circle);
    }
    code:after {
      content: counter(dummy,square);
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
