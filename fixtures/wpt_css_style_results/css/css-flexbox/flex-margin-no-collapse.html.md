# css/css-flexbox/flex-margin-no-collapse.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-margin-no-collapse.html"
}
```

## style[0]

```css

		#container {
			display: flex ;
			flex-direction: column;
			position: absolute;
			top: 100px;
			left: 10px;
			width: 200px;
			height: 300px;
		}

		.box {
			width: 100px;
			height: 100px;
			background-color: green;
			flex: none;
		}

		#box1 {
			margin: 50px 0;
		}

		#box2 {
			margin: 50px 0;
		}

		#red-box {
			position: absolute;
			top: 350px;
			left: 10px;
			width: 100px;
			height: 100px;
			background-color: red;
		}
	
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
