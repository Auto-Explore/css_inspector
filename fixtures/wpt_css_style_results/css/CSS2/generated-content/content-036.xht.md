# css/CSS2/generated-content/content-036.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/content-036.xht"
}
```

## style[0]

```css

            div:before
            {
                content: counters(test, ".", none);
                counter-increment: test;
                counter-reset: test;
                color: red;
            }
            div
            {
                border: 2px solid black;
                height: 30px;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
