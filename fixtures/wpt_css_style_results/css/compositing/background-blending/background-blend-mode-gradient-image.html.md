# css/compositing/background-blending/background-blend-mode-gradient-image.html

```json
{
  "format_version": 3,
  "file": "css/compositing/background-blending/background-blend-mode-gradient-image.html"
}
```

## style[0]

```css

			div {
				margin: 5px;
				width: 130px;
				height: 130px;
				background:  url('support/red.png') no-repeat 0 0 /100% 100%, linear-gradient(to right, lime 50%, blue 51%);
				/*lime: rgb(0,255,0);
				blue: rgb(0,0,255);*/
				display: block;
				float: left;
				background-blend-mode: multiply, normal;
			}
		
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-blend-mode”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
