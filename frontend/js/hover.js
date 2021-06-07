$(function() {
    $("#login").hover(function() {
		$("#login").removeClass("bg-c3");
            $("#login").addClass("hovered");
		$("#login").addClass("text-white-50");
        },
        function() {
            $("#login").removeClass("hovered");
		$("#login").addClass("bg-c3");
		$("#login").removeClass("text-white-50");
        });
	$("#back").hover(function() {
		$("#back").removeClass("bg-c3");
            $("#back").addClass("hovered");
		$("#back").addClass("text-white-50");
        },
        function() {
            $("#back").removeClass("hovered");
		$("#back").addClass("bg-c3");
		$("#back").removeClass("text-white-50");
        });
	$("#registr").hover(function() {
		$("#registr").removeClass("bg-c3");
            $("#registr").addClass("hovered");
		$("#registr").addClass("text-white-50");
        },
        function() {
            $("#registr").removeClass("hovered");
		$("#registr").addClass("bg-c3");
		$("#registr").removeClass("text-white-50");
        });
    $(".hover").hover(function() {
			$(this).removeClass("bg-c4");
            $(this).addClass("bg-c5");
			$(this).addClass("text-white-50");
        },
        function() {
            $(this).removeClass("bg-c5");
            $(this).addClass("bg-c4");
			$(this).removeClass("text-white-50");
        })
})