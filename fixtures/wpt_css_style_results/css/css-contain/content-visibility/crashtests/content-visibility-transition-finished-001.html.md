# css/css-contain/content-visibility/crashtests/content-visibility-transition-finished-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/crashtests/content-visibility-transition-finished-001.html"
}
```

## style[0]

```css

  #inner {
    /* Extremely short so that we can just do a double-rAF and
     * expect that this transition will have completed: */
    transition: opacity 0.01s;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
