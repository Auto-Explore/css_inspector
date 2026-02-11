# css/css-backgrounds/reference/css-box-shadow-ref-001.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/reference/css-box-shadow-ref-001.html"
}
```

## style[0]

```css

		.greenSquare-shadow{
            position: absolute;
			top:50px;
			left:50px;
            width: 100px;
            height: 100px;
			Border-bottom-right-radius: 50px 50px;
			Border-top-left-radius: 50px 50px;
			background-color:rgba(0, 255, 0, 1);
			/*box-shadow: 110px 110px 0px 10px #000000;*/
		}
		.black-shadow{
			position: absolute;
			top: 150px;
			left: 150px;
			width: 120px;
			height: 120px;
			Border-bottom-right-radius: 60px 60px;
			Border-top-left-radius: 60px 60px;
			background-color:black;
        }
        .container {
            position: absolute;
        }
         /* This div should only be visible if the test fails */
        .redSquare {
			position: absolute;
			top: 150px;
			left: 150px;
			width: 120px;
			height: 120px;
			Border-bottom-right-radius: 60px 60px;
			Border-top-left-radius: 60px 60px;
			background-color:red;
        }
    
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Invalid value for property “border-bottom-right-radius”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-top-left-radius”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-bottom-right-radius”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-top-left-radius”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-bottom-right-radius”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-top-left-radius”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
