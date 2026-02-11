# css/CSS2/positioning/right-offset-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/right-offset-002.xht"
}
```

## style[0]

```css

            #div1
            {
                background: blue;
                height: 100px;
                position: relative;
                width: 100px;
            }
            div div
            {
                background: white;
                height: 50px;
                position: relative;
                right: -50px;
                /* In this testcase, right offset is -50px;
                so, it will move toward the right from its
                normal in-flow position by a value of 50px. */
                width: 50px;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
