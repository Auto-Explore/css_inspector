# css/css-backgrounds/border-radius-sum-of-radii-002.htm

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-radius-sum-of-radii-002.htm"
}
```

## style[0]

```css

            div
            {
                border: 10px red solid;
                width: 80px;
                height: 80px;
            }
            #reference1
            {
                border-radius: 50px 50px 30px 30px;
            }
            #test1
            {
                margin-top: -100px;
                border-color: black;
                border-radius: 50px 50px 30px 30px;
            }
            #reference2
            {
                border-radius: 0 100px 0 0;
            }
            #test2
            {
                margin-top: -100px;
                border-color: black;
                border-radius: 30px 15000000px 30px 30px;
            }
            #reference3
            {
                border-radius: 0;
            }
            #test3
            {
                margin-top: -100px;
                border-color: black;
                border-radius: 30px -150px 30px 30px;
            }
            #reference4
            {
                width: 1000px;
                height: 500px;
                border-radius: 0.1px 0.1px 0.1px 0.1px;
            }
            #test4
            {
                width: 1000px;
                height: 500px;
                margin-top: -520px;
                border-color: black;
                border-radius: 0.1px 0.1px 0.1px 0.1px;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
