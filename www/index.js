import { Game } from "helibattle";

const canvas = document.getElementById("game-area");
const ctx = canvas.getContext('2d');
const new_game = Game.new();

const PLAYER_WIDTH = 20;
const PLAYER_HEIGHT = 10;

const renderLoop = () => {
		drawPlayers();
		drawMissiles();

		requestAnimationFrame(renderLoop);
};

const drawPlayers = () => {
		//player 1
		const player1 = new_game.get_p1();
		ctx.fillStyle = 'rgb(200,0,0)';
		ctx.fillRect(player1[0], player1[1], PLAYER_WIDTH,  PLAYER_HEIGHT);

		//player 2
		const player2 = new_game.get_p2;
		ctx.fillStyle = 'rgb(0,0,200)';
		ctx.fillRect(player2[0], player2[1], PLAYER_WIDTH,  PLAYER_HEIGHT);
};

const drawMissiles = () => {

};

renderLoop();
