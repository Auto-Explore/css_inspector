# css/css-backgrounds/border-image-width-001.htm

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-image-width-001.htm"
}
```

## style[0]

```css

            div.subtest
            {
                background-color:red;
                border: 10px double red;
                border-image-slice: 10;
                border-image-source: url("support/green_color.png");
                height: 100px;
                margin: 8px auto 8px 0px;
                width: 100px;
            }

            #subtest1
            {
                border-image-width: 5 4 3 2;
            }

            #subtest2
            {
                border-image-width: 50px 10px;
            }

            #subtest3
            {
                border-image-width: 40% 20%;
                height: 115px;
                width: 90px;
            }

            #subtest4
            {
                border-image-slice: 20 40;
                border-image-width: auto;
            }

            div div
            {
                background-color: lime;
            }

            div#subtest1 div , div#subtest3 div
            {
                height: 75px;
                margin: 20px 5px;
                width: 80px;
            }

            div#subtest2 div
            {
                height: 60px;
                margin: 20px 0px;
                width: 100px;
            }

            div#subtest4 div
            {
                height: 90px;
                margin: 5px 15px;
                width: 70px;
            }
        
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “border-image-width”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-image-width”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-image-width”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-image-slice”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
