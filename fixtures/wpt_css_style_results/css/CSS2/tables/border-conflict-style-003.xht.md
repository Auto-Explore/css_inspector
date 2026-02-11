# css/CSS2/tables/border-conflict-style-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-conflict-style-003.xht"
}
```

## style[0]

```css

            #table1
            {
                margin-bottom: 20px;
            }
            table
            {
                border-collapse: collapse;
            }
            td
            {
                border: 5px dashed lime;
                height: 3em;
                width: 3em;
            }
            #center, #table1 td
            {
                border: 5px double lime;
            }
            #top
            {
                border-top: 5px dashed red;
            }
            #left
            {
                border-left: 5px dashed red;
            }
            #bottom
            {
                border-bottom: 5px dashed red;
            }
            #right
            {
                border-right: 5px dashed red;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
