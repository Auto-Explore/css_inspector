# css/CSS2/generated-content/content-035.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/content-035.xht"
}
```

## style[0]

```css

            div:before
            {
                content: counters(test, ".", upper-alpha);
                counter-increment: test;
                counter-reset: test;
            }
            #div1
            {
                border: 2px solid black;
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
