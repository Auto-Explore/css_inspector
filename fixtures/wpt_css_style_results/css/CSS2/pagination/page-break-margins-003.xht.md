# css/CSS2/pagination/page-break-margins-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/pagination/page-break-margins-003.xht"
}
```

## style[0]

```css

	html, body {
                height: 100%;
                margin: 0;
                padding: 0;
    }
	body {
	    background: red;
	}
	div {
        background: white;
    }
    #top {
        height: 100%;
        margin-bottom: -2in;
    }
    div.one {
		page-break-before: avoid;
		page-break-after: always;
		background: white;
		height: 2in;
		margin-bottom: 2in;
	}
	div.two {
        background: white;
        margin: 2in 0 0 0;
        height: 100%;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
