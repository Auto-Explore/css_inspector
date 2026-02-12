# css/css-backgrounds/border-radius-shorthand-001.htm

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-radius-shorthand-001.htm"
}
```

## style[0]

```css

            div
            {
                width: 200px;
                height: 200px;
            }
            #reference1
            {
                border: 10px solid red;
                border-top-left-radius: 40px;
                border-top-right-radius: 40px;
                border-bottom-right-radius: 40px;
                border-bottom-left-radius: 40px;
            }
            #test1
            {
                margin-top: -220px;
                border: 10px solid black;
                border-radius : 40px;
            }
            #reference2
            {
                border: 10px solid red;
                border-top-left-radius: 2em 0.5em;
                border-top-right-radius: 1em 3em;
                border-bottom-right-radius: 4em 0.5em;
                border-bottom-left-radius: 1em 3em;
            }
            #test2
            {
                margin-top: -220px;
                border: 10px solid black;
                border-radius: 2em 1em 4em / 0.5em 3em;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
