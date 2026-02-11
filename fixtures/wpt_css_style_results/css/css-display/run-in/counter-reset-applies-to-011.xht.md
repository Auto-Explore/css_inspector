# css/css-display/run-in/counter-reset-applies-to-011.xht

```json
{
  "format_version": 3,
  "file": "css/css-display/run-in/counter-reset-applies-to-011.xht"
}
```

## style[0]

```css

            div
            {
                counter-reset: test 5;
                display: run-in;
            }
            div:before
            {
                content: counter(test);
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
