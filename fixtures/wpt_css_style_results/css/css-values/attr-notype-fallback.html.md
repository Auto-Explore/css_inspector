# css/css-values/attr-notype-fallback.html

```json
{
  "format_version": 3,
  "file": "css/css-values/attr-notype-fallback.html"
}
```

## style[0]

```css

  #one::after {
    content: attr(does-not-exist, "Fallback value");
  }

  #two::after {
    content: attr(does-exist, "Fallback value");
  }

  #three::after {
    content: attr(does-not-exist, invalid);
  }

  #four::after {
    content: attr(does-exist, invalid);
  }

  #five::after {
    content: attr(does-exist, "Fallback value");
  }

  #six::after {
    content: attr(does-exist, "Fallback value");
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
