# css/CSS2/tables/border-conflict-element-031.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-conflict-element-031.xht"
}
```

## style[0]

```css

            table
            {
                border-collapse: collapse;
                height: 2in;
                width: 2in;
            }
            #collapsing1
            {
                border-left: 5px solid black;
                border-right: 5px solid red;
            }
            #collapsing2
            {
                border-left: 5px solid red;
                border-right: 5px solid black;
            }
            col
            {
                border-bottom: 5px solid black;
                border-top: 5px solid black;
            }
            #test
            {
                border-left: 5px solid black;
                border-right: 5px solid black;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
