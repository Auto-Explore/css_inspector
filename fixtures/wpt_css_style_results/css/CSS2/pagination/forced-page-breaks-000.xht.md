# css/CSS2/pagination/forced-page-breaks-000.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/pagination/forced-page-breaks-000.xht"
}
```

## style[0]

```css

	@page {
		counter-increment: page;
		margin: 7%;
		@top-right {
           content: content: "Page " counter(page);
        }
	}
	@page :first {
		@top-center {
           content: "This test requires 7 pages";
        }
	}

	.breakBefore {page-break-before: always;}
	.breakAfter {page-break-after: always;}

	.breakBeforeLeft {page-break-before: left;}
	.breakBeforeRight {page-break-before: right;}

	.avoidBefore { page-break-before: avoid; }
	.avoidAfter { page-break-after: avoid; }
	.avoidInside { page-break-inside: avoid; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
