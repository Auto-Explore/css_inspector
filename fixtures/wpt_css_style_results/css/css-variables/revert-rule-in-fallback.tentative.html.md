# css/css-variables/revert-rule-in-fallback.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/revert-rule-in-fallback.tentative.html"
}
```

## style[0]

```css

  #child {
    --x:PASS;
    margin: 1px;
    padding-left: 1px;
  }
  #parent {
    --x:FAIL;
    margin: -1px;
    padding-left: -1px;
  }
  #child {
    --x: var(--unknown, revert-rule);
    margin: var(--unknown, revert-rule);
    padding-left: var(--unknown, revert-rule);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
