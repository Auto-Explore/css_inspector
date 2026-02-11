# css/css-display/run-in/counter-increment-applies-to-011.xht

```json
{
  "format_version": 3,
  "file": "css/css-display/run-in/counter-increment-applies-to-011.xht"
}
```

## style[0]

```css

            div
            {
                counter-increment: test 5;
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
