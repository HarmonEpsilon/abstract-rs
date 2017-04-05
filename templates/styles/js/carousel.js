$(window).on('load', function(){
		window.curslide =1;
		window.inTransition = false;
		var numslide = $("#carousel > div").length;
		$("#carousel > div:gt(0)").hide();
		$(".goto[value = 1]").css({"background-color":"Grey"});

		//Hides a specified slide
		function hideslide(hider){ 
			$('#carousel > div:nth-child('+ hider +')').hide();
		}

		//goes to a specified slide, handling all animation for that slide and the previous.
		function goslide(newslide, RL){
			if (inTransition){
					setTimeout(goslide,100, newslide, RL);
				}
			else{
				inTransition = true;
				var prevslide = curslide;

				if(RL == true){
					$('#carousel > div:nth-child('+ prevslide +')')
						.css({"z-index":0})
						.animate({marginLeft: -600},1100); //Figs: the first number needs to be the width of the carousel; the second is the time it takes for the animation to complete. If you change it, you also need to change the identical number in the next selector.
					$('#carousel > div:nth-child('+ newslide +')')
						.css({"margin-left":"600px","z-index":"1"})
						.show()
						.animate({marginLeft: -1},1100,function(){hideslide(prevslide); inTransition = false;}); //Figs: This'n.
				}

				else{
					$('#carousel > div:nth-child('+ prevslide +')') 
						.css({"z-index":0})
						.animate({marginLeft: 600},1100); //Figs: the first number needs to be the width of the carousel; the second is the time it takes for the animation to complete. If you change it, you also need to change the identical number in the next selector.
					$('#carousel > div:nth-child('+ newslide +')')
						.css({"margin-left":"-600px","z-index":"1"})
						.show()
						.animate({marginLeft: -1},1100,function(){hideslide(prevslide); inTransition = false;}); //Figs: This'n.			
				}
				$(".goto[value =" + curslide + "]").css({"background-color":"transparent"});
				curslide = newslide;
				$(".goto[value =" + newslide + "]").css({"background-color":"Grey"});
			}
		}

		//cycles the carousel automatically. Will run until cleared.
		var autoSlide = setInterval(function(){ 
			if(curslide >= numslide) {
				var nextslide = 1;
			}
			else {
				var nextslide = (curslide+1);
			}
			goslide(nextslide, true);

		},7500); //Figs: this number controls the time delay before transitions. Increase to increase the time spent on each slide.

		//handles the user manually moving one slide back via button input.
		$("#gonext").click(function() {
			$(".caronav").css({"pointer-events":"none"});
			if(curslide >= numslide) {
				var nextslide = 1;
			}
			else {
				var nextslide = curslide + 1;
			}
			goslide(nextslide, true);
			setTimeout( function() {
				$(".caronav").css({"pointer-events":"auto"});
			}, 1100); //Figs: this number also needs to be the time it takes the animation to complete.
		});

		$("#goprev").click(function() {
			$(".caronav").css({"pointer-events":"none"});
			if(curslide == 1) {
				var nextslide = 4;
			}
			else {
				var nextslide = curslide - 1;
			}
			goslide(nextslide, false);
			setTimeout( function() {
				$(".caronav").css({"pointer-events":"auto"});
			}, 1100); //Figs: this number also needs to be the time it takes the animation to complete.
		});

		//handles the user manually going to a slide via button input.
		$(".goto").click(function() {
			var dest = parseInt($(this).attr("value"), 10);
			if (curslide < dest) {	
				$(".caronav").css({"pointer-events":"none"});
				goslide(dest, true);
				setTimeout( function() {
					$(".caronav").css({"pointer-events":"auto"});
				}, 1100); //Figs: this number also needs to be the time it takes the animation to complete.
			}
			else if (curslide > dest) {
			$(".caronav").css({"pointer-events":"none"});
				goslide(dest, false);
				setTimeout( function() {
					$(".caronav").css({"pointer-events":"auto"});
				}, 1100); //Figs: this number also needs to be the time it takes the animation to complete.
			}
		});

		//Stops the carousel from auto cycling when a click event occurs within the parent div.
		$("#carouselcontent").click(function() {
			clearInterval(autoSlide);
		});


});