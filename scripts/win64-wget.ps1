# Usage: powershell -ExecutionPolicy Bypass -File win64-wget.ps1 -url http://www.example.com -depth 2 -username username -password password -outputfile output.txt
param(
    [string]$url,
    [int]$depth,
    [string]$username,
    [string]$password,
    [string]$outputfile
)

function Invoke-Recursive-WebRequest(
        [string]$url,
        [int]$depth,
        [string]$username,
        [string]$password,
        [string]$outputfile)
{


    $credential = $null

    # if url is not provided exit with an error message
    if (-not$url)
    {
        Write "Invoke-Recursive-WebRequest requires a url parameter"
        return
    }

    #if depth is not provided assume 0
    if (-not$depth)
    {
        $depth = 0
    }

    #if output file is not provided exit with an error message
    if (-not$outputfile)
    {
        Write "Invoke-Recursive-WebRequest requires an outputfile parameter"
        return
    }

    # If username and password are provided, create a PSCredential object
    # and pass it to Invoke-WebRequest
    if ($username -and $password)
    {
        $securePassword = ConvertTo-SecureString $password -AsPlainText -Force
        $credential = New-Object System.Management.Automation.PSCredential ($username, $securePassword)
    }


    # if a credential is provided, use it, otherwise, don't
    $webResponse = if ($credential)
    {
        Invoke-WebRequest -UseBasicParsing -Uri $url -Credential $credential -ErrorAction SilentlyContinue
    }
    else
    {
        Invoke-WebRequest -UseBasicParsing -Uri $url -ErrorAction SilentlyContinue
    }

    # if depth is provided and greater than 0 then parse response for links to download
    # recurse into each link depth number of times
    if ($depth -and $depth -gt 0)
    {
        $links = $webResponse.Links | Where-Object { $_.href -and $_.href -ne $url }
        foreach ($link in $links)
        {
            $linkoutputfile = $outputfile + "-" + [uri]::EscapeDataString($link.href).Replace("/", "-")
            Invoke-Recursive-WebRequest -url $link.href -depth ($depth - 1) -username $username -password $password -outputfile $linkoutputfile
        }
    }

    # save response to file
    $webResponse.Content | Out-File $outputfile

}

Invoke-Recursive-WebRequest -url $url -depth $depth -username $username -password $password -outputfile $outputfile
