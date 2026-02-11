# css/CSS2/pagination/page-break-after-010.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/pagination/page-break-after-010.xht"
}
```

## style[0]

```css

	@page {margin: 7%;}
	@page :right {
		@top-center
		{
			content: "[This page intentionally left blank]";
		}
	}
	@page :first {
		@top-center
		{
			content: "";
		}
	}
	p {
		page-break-after: left;
	}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
