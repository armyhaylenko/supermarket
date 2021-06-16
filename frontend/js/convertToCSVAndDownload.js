function convertToCSVAndDownload(data, filename) {
    const csv = $.csv.fromObjects(data);

    let downloadLink = document.createElement("a");
    const blob = new Blob([csv], { type: 'text/csv' });
    downloadLink.href = URL.createObjectURL(blob);
    downloadLink.download = filename + new Date().toISOString() + ".csv";
    document.body.appendChild(downloadLink);
    downloadLink.click();
    document.body.removeChild(downloadLink);
}